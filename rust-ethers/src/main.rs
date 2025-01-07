#![allow(unused)]
use ethers::prelude::*;
use std::collections::HashSet;

abigen!(Emitter, "../artifacts/contracts/Emitter.sol/Emitter.json");

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    polling_with_ethers().await?;
    Ok(())
}

async fn polling_with_ethers() -> Result<(), Box<dyn std::error::Error>> {
    let provider = Provider::<Http>::try_from("https://yellowstone-rpc.litprotocol.com")?;
    let contract_address = "0xEe1033c70701fe0ff133436AdD566c1877728e2b".parse::<Address>()?;
    let contract = Emitter::new(contract_address, provider.clone().into());
    let mut latest_block = provider.get_block_number().await?;
    let mut latest_processed_block = latest_block;

    println!("Polling for events...");
    loop {
        latest_block = provider.get_block_number().await?;
        if latest_processed_block < latest_block {
            let new_events = contract
                .events()
                .from_block(latest_processed_block + 1)
                .to_block(latest_block)
                .query()
                .await?;

            for event in new_events {
                println!("New event: {:?}", event);
            }
            latest_processed_block = latest_block;
        }
        tokio::time::sleep(tokio::time::Duration::from_secs(3)).await;
    }
}

// problematic because it sets up a stateful filter on the RPC node side, which can expire after 24 hours.
async fn listen_with_ethers() -> Result<(), Box<dyn std::error::Error>> {
    let provider = Provider::<Http>::try_from("https://yellowstone-rpc.litprotocol.com")?;
    let contract_address = "0xEe1033c70701fe0ff133436AdD566c1877728e2b".parse::<Address>()?;
    let contract = Emitter::new(contract_address, provider.clone().into());
    let events = contract.events();
    let mut stream = events.stream().await?;
    println!("Listening for events...");
    while let Some(event) = stream.next().await {
        println!("New event: {:?}", event);
    }
    Ok(())
}
