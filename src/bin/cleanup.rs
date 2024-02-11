use proton::prelude::{ProtonClient, Result};

const FN_NAME: &str = "[prepare]:";

#[tokio::main]
async fn main() -> Result<()> {
    println!("{} Start", FN_NAME);

    println!("{}Build client", FN_NAME);
    let client = ProtonClient::new("http://localhost:8123");

    println!("{} Delete Stream", FN_NAME);
    delete_stream(&client)
        .await
        .expect("[main]: Failed to delete Stream");

    println!("{} Stop", FN_NAME);

    Ok(())
}

pub async fn delete_stream(client: &ProtonClient) -> Result<()> {
    //  Drop a stream
    // https://docs.timeplus.com/proton-drop-stream
    client.execute_query("DROP STREAM some").await
}
