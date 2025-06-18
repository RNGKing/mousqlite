use tokio::net::{TcpSocket, TcpStream};
use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use tokio::io::AsyncWriteExt;
use mousqlite_types::{SqlRequest};

#[derive(Serialize, Deserialize)]
struct SqlTestData{
    data : Vec<String>
}

struct ByteData(Vec<u8>);

async fn get_test_query(path : &str) -> Result<SqlTestData>{
    let file_data = tokio::fs::read(path).await.context("Failed to read file")?;
    serde_json::from_slice(&file_data).context("Failed to parse json")
}

async fn run_test(byte_data: impl IntoIterator<Item=ByteData>) -> Result<()>{
    println!("Testing the tcp test");
    for data in byte_data{
        let mut tcp_client = TcpStream::connect("127.0.0.1:8080")
            .await.context("Failed to connect to host")?;
        tcp_client.write_all(&data.0).await.context("Failed to write data")?;
        tcp_client.flush().await.context("Failed to flush")?;
        tcp_client.shutdown().await.context("Failed to shutdown")?;
    }
    Ok(())
}

#[tokio::main]
async fn main() -> Result<()> {
    /*
    let query_test = get_test_query("test_data.json")
        .await
        .context("Failed to get query")?;

    let messages = query_test.data.iter().map(|query| {
        let sql_query = SqlRequest{request_id : 0, request : query.clone()};
        let request : Vec<u8> = NetworkRequest(sql_query).try_into()
            .context("Failed to serialize network request")?;
        let mut output = (request.len() as i128).to_be_bytes().to_vec();
        output.extend(request);
        Ok(ByteData(output))
    }).collect::<Result<Vec<ByteData>>>().context("Failed to parse query")?;
    run_test(messages).await.context("Failed to run query")
    */
    Ok(())
}
