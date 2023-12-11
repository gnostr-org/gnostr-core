// Copyright (c) 2022-2023 Yuki Kishimoto
// Copyright (c) 2023-2024 Rust Nostr Developers
// Distributed under the MIT software license

use std::ops::Deref;
use std::sync::Arc;

use nostr::message::subscription;
use nostr::JsonUtil;
use uniffi::{Enum, Object};

use crate::error::Result;
use crate::helper::unwrap_or_clone_arc;
use crate::{Event, EventId, PublicKey, Timestamp};

#[derive(Enum)]
pub enum Alphabet {
    A,
    B,
    C,
    D,
    E,
    F,
    G,
    H,
    I,
    J,
    K,
    L,
    M,
    N,
    O,
    P,
    Q,
    R,
    S,
    T,
    U,
    V,
    W,
    X,
    Y,
    Z,
}

impl From<Alphabet> for subscription::Alphabet {
    fn from(value: Alphabet) -> Self {
        match value {
            Alphabet::A => Self::A,
            Alphabet::B => Self::B,
            Alphabet::C => Self::C,
            Alphabet::D => Self::D,
            Alphabet::E => Self::E,
            Alphabet::F => Self::F,
            Alphabet::G => Self::G,
            Alphabet::H => Self::H,
            Alphabet::I => Self::I,
            Alphabet::J => Self::J,
            Alphabet::K => Self::K,
            Alphabet::L => Self::L,
            Alphabet::M => Self::M,
            Alphabet::N => Self::N,
            Alphabet::O => Self::O,
            Alphabet::P => Self::P,
            Alphabet::Q => Self::Q,
            Alphabet::R => Self::R,
            Alphabet::S => Self::S,
            Alphabet::T => Self::T,
            Alphabet::U => Self::U,
            Alphabet::V => Self::V,
            Alphabet::W => Self::W,
            Alphabet::X => Self::X,
            Alphabet::Y => Self::Y,
            Alphabet::Z => Self::Z,
        }
    }
}

#[derive(Clone, Object)]
pub struct Filter {
    inner: nostr::Filter,
}

impl Deref for Filter {
    type Target = nostr::Filter;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl From<nostr::Filter> for Filter {
    fn from(f: nostr::Filter) -> Self {
        Self { inner: f }
    }
}

#[uniffi::export]
impl Filter {
    #[uniffi::constructor]
    pub fn new() -> Arc<Self> {
        Arc::new(Self {
            inner: nostr::Filter::new(),
        })
    }

    pub fn id(self: Arc<Self>, id: Arc<EventId>) -> Arc<Self> {
        let mut builder = unwrap_or_clone_arc(self);
        builder.inner = builder.inner.id(**id);
        Arc::new(builder)
    }

    pub fn ids(self: Arc<Self>, ids: Vec<Arc<EventId>>) -> Arc<Self> {
        let mut builder = unwrap_or_clone_arc(self);
        builder.inner = builder.inner.ids(ids.into_iter().map(|id| **id));
        Arc::new(builder)
    }

    pub fn author(self: Arc<Self>, author: Arc<PublicKey>) -> Arc<Self> {
        let mut builder = unwrap_or_clone_arc(self);
        builder.inner = builder.inner.author(**author);
        Arc::new(builder)
    }

    pub fn authors(self: Arc<Self>, authors: Vec<Arc<PublicKey>>) -> Arc<Self> {
        let mut builder = unwrap_or_clone_arc(self);
        builder.inner = builder.inner.authors(authors.into_iter().map(|pk| **pk));
        Arc::new(builder)
    }

    pub fn kind(self: Arc<Self>, kind: u64) -> Arc<Self> {
        let mut builder = unwrap_or_clone_arc(self);
        builder.inner = builder.inner.kind(kind.into());
        Arc::new(builder)
    }

    pub fn kinds(self: Arc<Self>, kinds: Vec<u64>) -> Arc<Self> {
        let mut builder = unwrap_or_clone_arc(self);
        builder.inner = builder.inner.kinds(kinds.into_iter().map(|k| k.into()));
        Arc::new(builder)
    }

    pub fn event(self: Arc<Self>, event_id: Arc<EventId>) -> Arc<Self> {
        let mut builder = unwrap_or_clone_arc(self);
        builder.inner = builder.inner.event(event_id.as_ref().into());
        Arc::new(builder)
    }

    pub fn events(self: Arc<Self>, ids: Vec<Arc<EventId>>) -> Arc<Self> {
        let mut builder = unwrap_or_clone_arc(self);
        builder.inner = builder
            .inner
            .events(ids.into_iter().map(|id| id.as_ref().into()));
        Arc::new(builder)
    }

    pub fn pubkey(self: Arc<Self>, pubkey: Arc<PublicKey>) -> Arc<Self> {
        let mut builder = unwrap_or_clone_arc(self);
        builder.inner = builder.inner.pubkey(*pubkey.as_ref().deref());
        Arc::new(builder)
    }

    pub fn pubkeys(self: Arc<Self>, pubkeys: Vec<Arc<PublicKey>>) -> Arc<Self> {
        let mut builder = unwrap_or_clone_arc(self);
        builder.inner = builder
            .inner
            .pubkeys(pubkeys.into_iter().map(|id| *id.as_ref().deref()));
        Arc::new(builder)
    }

    pub fn identifier(self: Arc<Self>, identifier: String) -> Arc<Self> {
        let mut builder = unwrap_or_clone_arc(self);
        builder.inner = builder.inner.identifier(identifier);
        Arc::new(builder)
    }

    pub fn search(self: Arc<Self>, text: String) -> Arc<Self> {
        let mut builder = unwrap_or_clone_arc(self);
        builder.inner = builder.inner.search(text);
        Arc::new(builder)
    }

    pub fn since(self: Arc<Self>, timestamp: Arc<Timestamp>) -> Arc<Self> {
        let mut builder = unwrap_or_clone_arc(self);
        builder.inner = builder.inner.since(*timestamp.as_ref().deref());
        Arc::new(builder)
    }

    pub fn until(self: Arc<Self>, timestamp: Arc<Timestamp>) -> Arc<Self> {
        let mut builder = unwrap_or_clone_arc(self);
        builder.inner = builder.inner.until(*timestamp.as_ref().deref());
        Arc::new(builder)
    }

    pub fn limit(self: Arc<Self>, limit: u64) -> Arc<Self> {
        let mut builder = unwrap_or_clone_arc(self);
        builder.inner = builder.inner.limit(limit as usize);
        Arc::new(builder)
    }

    pub fn custom_tag(self: Arc<Self>, tag: Alphabet, content: Vec<String>) -> Arc<Self> {
        let mut builder = unwrap_or_clone_arc(self);
        builder.inner = builder.inner.custom_tag(tag.into(), content);
        Arc::new(builder)
    }

    pub fn match_event(&self, event: Arc<Event>) -> bool {
        self.inner.match_event(event.as_ref().deref())
    }

    #[uniffi::constructor]
    pub fn from_json(json: String) -> Result<Arc<Self>> {
        Ok(Arc::new(Self {
            inner: nostr::Filter::from_json(json)?,
        }))
    }

    pub fn as_json(&self) -> String {
        self.inner.as_json()
    }
}