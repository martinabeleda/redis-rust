use mini_redis::{client, Result};

#[tokio::main]
async fn main() -> Result<()> {
    // Open a connection to mini redis
    let mut client = client::connect("localhost:6379").await?;

    client.set("hello", "world".into()).await?;

    let result = client.get("hello").await?;

    println!("Got value from server; result={:?}", result);

    Ok(())
}
