use dotenv::dotenv;
use solana_client::{rpc_client::RpcClient, rpc_config::RpcBlockConfig};
use solana_transaction_status::{EncodedConfirmedBlock, UiTransactionEncoding};

fn get_block(client: &RpcClient, block_num: u64) -> EncodedConfirmedBlock {
    log::debug!("Getting block number: {}", block_num);

    let config = RpcBlockConfig {
        encoding: Some(UiTransactionEncoding::Base64),
        max_supported_transaction_version: Some(0),
        ..Default::default()
    };
    
    let block = client.get_block_with_config(block_num, config);
    let encoded_block: EncodedConfirmedBlock = block.unwrap().into();

    encoded_block
}

fn main() {
    dotenv().ok();
    env_logger::init();

    log::info!("Count solana transactions per second");

    // let client = RpcClient::new("https://solana-api.projectserum.com");
    let client = RpcClient::new("https://api.devnet.solana.com");
    let solana_version = client.get_version().unwrap().solana_core;
    log::info!("Solana version: {}", &solana_version);

    let latest_block_number = client.get_slot().unwrap();
    let block = get_block(&client, latest_block_number);
    log::info!("Transactions count: {}", block.transactions.len());
}
