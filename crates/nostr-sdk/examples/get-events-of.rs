// Copyright (c) 2022-2023 Yuki Kishimoto
// Copyright (c) 2023-2024 Rust Nostr Developers
// Distributed under the MIT software license

use std::time::Duration;

use nostr_sdk::prelude::*;

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();

    let public_key = XOnlyPublicKey::from_bech32(
        "npub1080l37pfvdpyuzasyuy2ytjykjvq3ylr5jlqlg7tvzjrh9r8vn3sf5yaph",
    )?;

    let client = Client::default();
    client.add_relay("wss://relay.damus.io").await?;
    client.add_relay("wss://nostr.wine").await?;
    client.add_relay("wss://relay.nostr.info").await?;

    client.connect().await;

    let filter = Filter::new().author(public_key).kind(Kind::Metadata);
    let events = client
        .get_events_of(vec![filter], Some(Duration::from_secs(10)))
        .await;
    println!("{events:#?}");

    Ok(())
}