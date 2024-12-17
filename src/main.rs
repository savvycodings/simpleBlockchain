mod blockchain;

use blockchain::chain::Blockchain;
use tokio::time::{sleep, Duration};

#[tokio::main]
async fn main(){
    let mut blockchain = Blockchain::new();

    for i in 1..=3{
        println!("Adding block {}...", i);
        blockchain.add_block(format!("Block data {}", i));
        sleep(Duration::from_secs(2)).await;
    }

    println!("Blockchain:");
    println!("{}", serde_json::to_string_pretty(&blockchain.chain).unwrap());

}