use ai_node::AINode;
use tokio;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize the AI node
    let blockchain_names = vec!["Ethereum".to_string(), "Solana".to_string()];
    let node_id = "node-123";
    let node = AINode::new(node_id.to_string(), blockchain_names);

    // Ensure that the `start` method of `AINode` is async
    node.start().await?;

    // Spawn asynchronous tasks for data collection
    let data_collection_task = tokio::spawn(async {
        if let Err(e) = data_collection::collect_dapp_interaction_data().await {
            eprintln!("Error collecting dApp interaction data: {}", e);
        }

        if let Err(e) = data_collection::collect_transaction_data().await {
            eprintln!("Error collecting transaction data: {}", e);
        }
    });

    // Spawn performance monitoring task
    let performance_task = tokio::spawn(async {
        if let Err(e) = node.monitor_performance().await {
            eprintln!("Error monitoring performance: {}", e);
        }
    });

    // Wait for the tasks to complete
    data_collection_task.await??;
    performance_task.await??;

    // Gracefully exit the main function
    Ok(())
}
