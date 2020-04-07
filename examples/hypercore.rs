use async_std::io;
use async_std::net::TcpStream;
use async_std::prelude::*;
use async_std::sync::{Arc, Mutex, RwLock};
use async_std::task;
use async_trait::async_trait;
use hypercore::{Feed, Node, NodeTrait, Proof, PublicKey, Signature, Storage};
use pretty_hash::fmt as pretty_fmt;
use random_access_memory::RandomAccessMemory;
use random_access_storage::RandomAccess;
use std::collections::HashMap;
use std::env;
use std::fmt::Debug;
use std::io::Result;

use hypercore_protocol::schema::*;
use hypercore_protocol::{discovery_key, ProtocolBuilder};
use hypercore_protocol::{Channel, ChannelHandler, StreamContext, StreamHandler};

mod util;
use util::{tcp_client, tcp_server};

fn main() {
    util::init_logger();
    if env::args().count() < 3 {
        usage();
    }
    let mode = env::args().nth(1).unwrap();
    let port = env::args().nth(2).unwrap();
    let address = format!("127.0.0.1:{}", port);

    let key = env::args().nth(3);
    let key = key.map_or(None, |key| hex::decode(key).ok());

    task::block_on(async move {
        let mut feedstore: FeedStore<RandomAccessMemory> = FeedStore::new();
        if let Some(key) = key {
            let storage = Storage::new_memory().await.unwrap();
            let public_key = PublicKey::from_bytes(&key).unwrap();

            let feed = Feed::builder(public_key, storage).build().unwrap();
            // let feed = Feed::with_storage(storage).await.unwrap();

            let feed_wrapper: FeedWrapper<RandomAccessMemory> = FeedWrapper::from_feed(feed);
            feedstore.add(feed_wrapper);
        }
        let feedstore = Arc::new(feedstore);

        let result = match mode.as_ref() {
            "server" => tcp_server(address, onconnection, feedstore).await,
            "client" => tcp_client(address, onconnection, feedstore).await,
            _ => panic!(usage()),
        };
        util::log_if_error(&result);
    });
}

/// Print usage and exit.
fn usage() {
    println!("usage: cargo run --example basic -- [client|server] [port] [key]");
    std::process::exit(1);
}

async fn append<T>(feed: &mut Feed<T>, content: &[u8])
where
    T: RandomAccess<Error = Box<dyn std::error::Error + Send + Sync>> + Debug + Send,
{
    feed.append(content).await.unwrap();
}

async fn print<T>(feed: &Arc<Mutex<Feed<T>>>)
where
    T: RandomAccess<Error = Box<dyn std::error::Error + Send + Sync>> + Debug + Send,
{
    let mut feed = feed.lock().await;
    let len = feed.len();
    println!("Feed len: {}", len);
    for i in (len - 1)..len {
        let node = feed.get(i).await.unwrap();
        if let Some(value) = node {
            println!("{}: {:?}", i, String::from_utf8(value).unwrap());
        } else {
            println!("{}: {:?}", i, "NONE");
        }
    }
}

// fn main() {
//     task::block_on(task::spawn(async {
//         let mut feed = Feed::default();

//         append(&mut feed, b"hello").await;
//         append(&mut feed, b"world").await;
//         print(&mut feed).await;
//     }));
// }

// The onconnection handler is called for each incoming connection (if server)
// or once when connected (if client).
async fn onconnection<T: 'static>(
    stream: TcpStream,
    is_initiator: bool,
    feedstore: Arc<FeedStore<T>>,
) -> Result<()>
where
    T: RandomAccess<Error = Box<dyn std::error::Error + Send + Sync>> + Debug + Send,
{
    let mut protocol = ProtocolBuilder::new(is_initiator)
        .set_handlers(feedstore)
        .from_stream(stream);

    protocol.listen().await
}

struct FeedStore<T>
where
    T: RandomAccess<Error = Box<dyn std::error::Error + Send + Sync>> + Debug + Send,
{
    feeds: HashMap<String, Arc<FeedWrapper<T>>>,
}
impl<T> FeedStore<T>
where
    T: RandomAccess<Error = Box<dyn std::error::Error + Send + Sync>> + Debug + Send,
{
    pub fn new() -> Self {
        let feeds = HashMap::new();
        Self { feeds }
    }

    pub fn add(&mut self, feed: FeedWrapper<T>) {
        let hdkey = hex::encode(&feed.discovery_key);
        self.feeds.insert(hdkey, Arc::new(feed));
    }

    pub fn get(&self, discovery_key: &[u8]) -> Option<&Arc<FeedWrapper<T>>> {
        let hdkey = hex::encode(discovery_key);
        self.feeds.get(&hdkey)
    }
}

/// We implement the StreamHandler trait on the FeedStore.
#[async_trait]
impl<T: 'static> StreamHandler for FeedStore<T>
where
    T: RandomAccess<Error = Box<dyn std::error::Error + Send + Sync>> + Debug + Send,
{
    async fn on_discoverykey(
        &self,
        protocol: &mut StreamContext,
        discovery_key: &[u8],
    ) -> Result<()> {
        log::trace!("open discovery_key: {}", pretty_fmt(discovery_key).unwrap());
        if let Some(feed) = self.get(discovery_key) {
            protocol.open(feed.key.clone(), feed.clone()).await
        } else {
            Ok(())
        }
    }
}

/// A Feed is a single unit of replication, an append-only log.
/// This toy feed can only read sequentially and does not save or buffer anything.
#[derive(Debug, Clone)]
struct FeedWrapper<T>
where
    T: RandomAccess<Error = Box<dyn std::error::Error + Send + Sync>> + Debug + Send,
{
    key: Vec<u8>,
    discovery_key: Vec<u8>,
    state: Arc<RwLock<FeedState>>,
    feed: Arc<Mutex<Feed<T>>>,
}
impl FeedWrapper<RandomAccessMemory> {
    pub fn new(key: Vec<u8>) -> Self {
        let feed: Feed<RandomAccessMemory> = Feed::default();
        FeedWrapper {
            discovery_key: discovery_key(&key),
            key,
            state: Arc::new(RwLock::new(FeedState::default())),
            feed: Arc::new(Mutex::new(feed)),
        }
    }

    pub fn from_feed(feed: Feed<RandomAccessMemory>) -> Self {
        let key = feed.public_key().to_bytes();
        FeedWrapper {
            discovery_key: discovery_key(&key),
            key: key.to_vec(),
            state: Arc::new(RwLock::new(FeedState::default())),
            feed: Arc::new(Mutex::new(feed)),
        }
    }
}

/// The Feed structs implements the ChannelHandler trait.
/// This allows to pass a Feed struct into the protocol when `open`ing a channel,
/// making it the handler for all messages that arrive on this channel.
/// The trait fns all receive a `channel` arg that allows to send messages over
/// the current channel.
#[async_trait]
impl<T> ChannelHandler for FeedWrapper<T>
where
    T: RandomAccess<Error = Box<dyn std::error::Error + Send + Sync>> + Debug + Send,
{
    async fn on_open<'a>(&self, channel: &mut Channel<'a>, discovery_key: &[u8]) -> Result<()> {
        log::info!("open channel {}", pretty_fmt(&discovery_key).unwrap());
        let msg = Want {
            start: 0,
            length: None,
        };
        channel.want(msg).await
    }

    async fn on_have<'a>(&self, channel: &mut Channel<'a>, msg: Have) -> Result<()> {
        let mut state = self.state.write().await;
        // Check if the remote announces a new head.
        log::info!(
            "receive have: {} (remote_head {:?})",
            msg.start,
            state.remote_head
        );
        if state.remote_head == None {
            state.remote_head = Some(msg.start);
            let msg = Request {
                index: 0,
                bytes: None,
                hash: None,
                nodes: None,
            };
            channel.request(msg).await?;
        } else if let Some(remote_head) = state.remote_head {
            if remote_head < msg.start {
                state.remote_head = Some(msg.start)
            }
        }
        Ok(())
    }

    async fn on_data<'a>(&self, channel: &mut Channel<'a>, msg: Data) -> Result<()> {
        let state = self.state.read().await;
        log::info!(
            "receive data: idx {}, {} bytes (remote_head {:?})",
            msg.index,
            msg.value.as_ref().map_or(0, |v| v.len()),
            state.remote_head
        );

        let mut feed = self.feed.lock().await;
        // let len = match msg.value {
        //     Some(value) => value.len(),
        //     None => 0,
        // };

        let (value, len): (Option<&[u8]>, usize) = match msg.value.as_ref() {
            None => (None, 0),
            Some(value) => {
                eprintln!(
                    "recv {} {}",
                    msg.index,
                    String::from_utf8(value.clone()).unwrap()
                );
                (Some(value), value.len())
            }
        };

        let signature = match msg.signature {
            Some(bytes) => Some(Signature::from_bytes(&bytes).unwrap()),
            None => None,
        };
        let nodes = msg
            .nodes
            .iter()
            .map(|n| Node::new(n.index, n.hash.clone(), n.size))
            .collect();
        let proof = Proof {
            index: msg.index,
            nodes,
            signature,
        };

        // eprintln!("PUT {:?} {:?} {:?}", msg.index, value, proof);
        // feed.put(msg.index, None, proof.clone()).await.unwrap();
        // let index = match msg.index {
        //     0 => 0,
        //     _ => msg.index - 1,
        // };
        // println!("proof {:?}", proof);
        println!("idx {} data {:?}", msg.index, &value);
        println!(
            "Proof: {:#?}",
            proof
                .clone()
                .nodes
                .iter()
                .map(|n| {
                    let n = n.clone();
                    format!("index {} len {} parent {}", n.index(), n.len(), n.parent())
                })
                .collect::<Vec<String>>()
        );
        // feed.put(msg.index, None, proof.clone()).await.unwrap();
        feed.put(msg.index, value, proof.clone()).await.unwrap();

        let i = msg.index;
        let node = feed.get(i).await.unwrap();
        if let Some(value) = node {
            println!("get {}: {:?}", i, String::from_utf8(value).unwrap());
        } else {
            println!("get {}: {:?}", i, "NONE");
        }

        println!("PUT OK!");
        // drop(feed);
        // print(&self.feed).await;
        // if let Some(value) = msg.value {
        //     let feed = self.feed.lock().await;
        //     let mut stdout = io::stdout();
        //     stdout.write_all(&value).await?;
        //     stdout.flush().await?;
        // }
        println!("req next");

        let next = msg.index + 1;
        if let Some(remote_head) = state.remote_head {
            if remote_head >= next {
                // Request next data block.
                let msg = Request {
                    index: next,
                    bytes: None,
                    hash: None,
                    nodes: None,
                };
                channel.request(msg).await?;
            }
        }

        Ok(())
    }
}

/// A FeedState stores the head seq of the remote.
/// This would have a bitfield to support sparse sync in the actual impl.
#[derive(Debug)]
struct FeedState {
    remote_head: Option<u64>,
}
impl Default for FeedState {
    fn default() -> Self {
        FeedState { remote_head: None }
    }
}
