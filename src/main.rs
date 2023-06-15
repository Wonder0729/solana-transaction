use dotenv::dotenv;
use solana_client::rpc_client::RpcClient;

fn main() {
    dotenv().ok();
    env_logger::init();

    log::info!("Count solana transactions per second");

    // let client = RpcClient::new("https://solana-api.projectserum.com");
    let client = RpcClient::new("https://api.devnet.solana.com");
    let solana_version = client.get_version().unwrap().solana_core;
    log::info!("Solana version: {}", &solana_version);
}