use std::collections::VecDeque;

use mousqlite_types::SqlRequest;
use tmq::{request, Context, Multipart};
use anyhow::{Context as _, Result};
use zmq::Message;


#[tokio::main]
async fn main() -> Result<()> {
    println!("Attempting to make the system run");
    let mut send_sock = request(&Context::new())
        .connect("tcp://127.0.0.1:3000")
        .context("Failed to connect to the system")?;
    
    let identifier = "1";
    let request_id = "A1B2C3";
    let request_body: String =  match (SqlRequest{
        request_id : 10,
        request : "CREATE TABLE test (number INTEGER);".to_string()
        }).try_into() {
        Ok(string_val) => string_val,
        Err(_) => {
            return Err(anyhow::anyhow!("Failed to convert object to string"));
        }
    };
    let message : VecDeque<Message>= vec![identifier.into(), request_id.into(), request_body.as_str().into()].into();
    let recv_sock = send_sock.send(Multipart::from_iter(message)).await?;
    let (_, send) = recv_sock.recv().await?;
    println!("Success send");
    send_sock = send;
    Ok(())
}
