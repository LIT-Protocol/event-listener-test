#![allow(unused)]
use ethers::prelude::*;

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
    let mut latest_processed_block = U64::from(0);
    println!("Polling for events...");
    loop {
        latest_block = provider.get_block_number().await?;
        if latest_processed_block < latest_block {
            let events = contract.events();
            let new_events = events.from_block(latest_block).query().await?;
            for event in new_events {
                println!("New event: {:?}", event);
            }
            latest_processed_block = latest_block;
        }
        tokio::time::sleep(tokio::time::Duration::from_secs(3)).await;
    }
    Ok(())
}
