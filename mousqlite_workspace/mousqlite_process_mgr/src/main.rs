mod process_manager;

use std::collections::VecDeque;
use std::os::fd::IntoRawFd;
use mousqlite_types::SqlRequest;
use tmq::{request, Context, Multipart};
use anyhow::{Context as _, Result, anyhow};
use zmq::Message;

/*
    TODO:
    * Create a process manager pool that controls access to processes in an async way 

 */


use axum::{
    routing::{get, post},
    http::StatusCode,
    Json, Router, response::Html
};
use axum::response::IntoResponse;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
struct HealthMessage{
    message: String,
    status : String
}

struct SqlProcessMgr{
    
}

#[derive(Clone, Copy)]
struct AppState{
    
}

#[tokio::main]
async fn main() -> Result<()> {

    let app = Router::new()
        .route("/", get(health))
        .route("/query", get(query_database));
    let listener = tokio::net::TcpListener::bind(format!("127.0.0.1:{}", 7878)).await?;
    axum::serve(listener, app).await?;
    // make the main control loop
    // client communicates via http?
    /*
    println!("Attempting to make the system run");
    let mut send_sock = request(&Context::new())
        .connect("tcp://127.0.0.1:3000")
        .context("Failed to connect to the system")?;
    
    let identifier = "1";
    let request_id = "A1B2C3";
    let request_body: String =  match (SqlRequest{
        request_id : 10,
        request : "INSERT INTO test (numbers".to_string()
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

     */

    Ok(())
}

async fn health() -> impl IntoResponse{
    let msg = HealthMessage { message: "healthy".to_string(), status: "system operational".to_string() };
    (StatusCode::OK,Json(msg))
}

async fn query_database() -> impl IntoResponse {
    (StatusCode::OK, Html("Testing"))
}