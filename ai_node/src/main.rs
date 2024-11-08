mod ai_node;
mod data_collection;
mod performance_monitor;
mod model_training;

use ai_node::AINode;
use tokio;

#[tokio::main]
async fn main() {
    // Initialize the AI node
    let node = AINode::new(
        "node-123".to_string(),
        vec!["Ethereum".to_string(), "Solana".to_string()],
    );
    node.start();

    // Collect data
    data_collection::collect_dapp_interaction_data().await;
    data_collection::collect_transaction_data().await;

    // Monitor performance
    node.monitor_performance();
}

