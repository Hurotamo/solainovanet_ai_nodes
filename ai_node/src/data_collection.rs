use web3::futures::StreamExt;
use web3::types::FilterBuilder;

pub async fn collect_dapp_interaction_data() {
    let transport = web3::transports::Http::new("https://sepolia.infura.io/v3/609f442bf0e14fa08396c80f39b1a592").unwrap();
    let web3 = web3::Web3::new(transport);

    let filter = FilterBuilder::default().build();
    let logs_stream = web3.eth_subscribe().subscribe_logs(filter).await.unwrap();

    logs_stream.for_each(|log| async {
        match log {
            Ok(log) => println!("New log: {:?}", log),
            Err(e) => eprintln!("Error: {}", e),
        }
    }).await;
}

pub async fn collect_transaction_data() {
    let transport = web3::transports::Http::new("https://sepolia.infura.io/v3/609f442bf0e14fa08396c80f39b1a592").unwrap();
    let web3 = web3::Web3::new(transport);

    let block_number = web3.eth().block_number().await.unwrap();
    println!("Current block number: {:?}", block_number);

    // Fetch and process transaction data
}

