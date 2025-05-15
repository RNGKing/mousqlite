use tokio::net::{TcpSocket, TcpStream};
use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use tokio::io::AsyncWriteExt;
use mousqlite_types::{RequestType, SqlRequest};

#[derive(Serialize, Deserialize)]
struct SqlTestData{
    data : Vec<String>
}

async fn delete_file(path : String) -> Result<()> {
    let path = std::path::Path::new(&path);
    tokio::fs::remove_file(path).await?;
    Ok(())
}

#[tokio::main]
async fn main() -> Result<()>{
    println!("Testing the tcp test");
    let mut tcp_client = TcpStream::connect("127.0.0.1:8080")
        .await.context("Failed to connect to host")?;

    let path = "/Users/test/projects/rust_projects/mousqlite/mousqlite_workspace/database/test.db".to_string();
    delete_file(path).await.context("Failed to delete file")?;
    let test_queries = 

    let query = "SELECT * FROM table";
    let sql = SqlRequest{
        request_id : 0,
        request : query.to_string(),
    };
    let message : Vec<u8> = RequestType::try_into(RequestType::NetworkRequest(sql))?;
    let length = (message.len() as i128).to_be_bytes().to_vec();
    let mut data = vec![];
    data.extend(length);
    data.extend(message);
    tcp_client.write_all(&data).await?;
    Ok(())
}
