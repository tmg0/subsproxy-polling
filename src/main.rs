use std::env;
use dotenv::dotenv;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
struct Server {
    id: String,
    address: String,
    #[serde(rename = "subscriptionId")]
    subscription_id: String,
    #[serde(rename = "createdAt")]
    created_at: String,
    #[serde(rename = "updatedAt")]
    updated_at: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();
    let url = env::var("SUBSPROXY_URL").unwrap();
    let servers = reqwest::get(&url).await?.json::<Vec<Server>>().await?;

    for server in servers {
        println!("ID: {}", server.id);
        println!("Address: {}", server.address);
        println!("Subscription ID: {}", server.subscription_id);
        println!("Created At: {}", server.created_at);
        println!("Updated At: {}", server.updated_at);
        println!();
    }

    Ok(())
}
