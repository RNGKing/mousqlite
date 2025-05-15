use std::{collections::VecDeque, ops::Not};

use anyhow::{Context as _, Result};
use mousqlite_types::{ColumnData, DataRow, RequestType, SqlRequest, SqlResponse};
use pest::Parser;
use pest_derive::Parser;
use rusqlite::types::ValueRef;
use tokio_rusqlite::Connection;
use tmq::{Context, Multipart};
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::Mutex;
use tokio::time::timeout;
use std::future::Future;
use std::net::SocketAddr;
use tokio::io::AsyncReadExt;
use tokio::net::TcpStream;

#[derive(Parser)]
#[grammar = "arg_parse_validation.pest"]
struct ArgParser;

struct HostUrl(String);
struct TcpString(String);

impl TryInto<TcpString> for HostUrl{
    type Error = anyhow::Error;

    fn try_into(self) -> Result<TcpString, anyhow::Error> {
        let output = format!("tcp://{}", self.0);
        Ok(TcpString(output))
    }
} 

fn try_parse_host_url(input : &String) -> Result<HostUrl>{

    let _ = ArgParser::parse(Rule::host_url, input.as_str())
        .context(format!("Failed to parse {}",input))?;
    Ok(HostUrl(input.to_owned()))
}

fn convert_valueref_to_columndata(col_data : ValueRef) -> Result<ColumnData>{
    match col_data {
        ValueRef::Null => Ok(ColumnData::Null),
        ValueRef::Integer(int) => Ok(ColumnData::Integer(int)),
        ValueRef::Real(real) => Ok(ColumnData::Real(real)),
        ValueRef::Text(items) => {
            let string_val = String::from_utf8(items.to_vec())
                .context("Failed to convert bytes into string")?;
            Ok(ColumnData::Text(string_val))
        },
        ValueRef::Blob(items) => Ok(ColumnData::Blob(items.to_vec())),
    }
}

fn get_row_data( row : &rusqlite::Row, column_count : usize) -> std::result::Result<Vec<ColumnData>, rusqlite::Error>{
    let mut row_data = vec![];
    for columm in 0 .. column_count {
        let data = row.get_ref(columm)?;
        let converted = match convert_valueref_to_columndata(data){
            Ok(col_data) => col_data,
            Err(_) => return Err(rusqlite::Error::InvalidQuery),
        };
        row_data.push(converted);
    }
    Ok(row_data)
}

fn extract_multipart(mut msg :  Multipart) -> Result<(String, String, String)> {

    let identifier = msg
        .pop_front().context("Failed to extract identifier from the message")?
        .as_str().context("Failed to convert identifier to string")?.to_string();
    let request_id = msg.pop_front()
        .context("Failed to get request id")?
        .as_str()
        .context("Failed to convert identifier as string")?.to_string();
    let request_body = msg.pop_front()
        .context("Failed to get request body")?
        .as_str().context("Failed to convert to string")?.to_string();
    let output = (identifier, request_id, request_body);
    Ok(output)
}

async fn execute_sql_request(req : SqlRequest, conn : &Arc<Mutex<Connection>>) -> Result<SqlResponse>{
    let connection = conn.lock().await;
    let output = connection.call(move | conn| {
        let mut stmt = conn.prepare(&req.request)?;
        let column_metadata = stmt
            .column_names()
            .iter().map(|item|{item.to_string()})
            .collect::<Vec<String>>();
        let col_count = stmt.column_count();
        let references = stmt
            .query_map([],
            |row|{
                get_row_data(row, col_count)
            })?;
        let mut output_vec = vec![];
        for row in references{
            match row {
                Ok(row_data) => {
                    output_vec.push(DataRow(row_data))
                },
                Err(_) => {return Err(tokio_rusqlite::Error::ConnectionClosed); }
            }
        }
        Ok(SqlResponse {
            request_id : 0,
            column_names : column_metadata,
            row_data : output_vec
        })
    }).await;
    match output {
        Ok(response) => Ok(response),
        Err(_) => Err(anyhow::anyhow!("Failed to get the SqlResponse from user query")),
    }
}

async fn handle_connection(mut stream : TcpStream, addr: SocketAddr) -> Result<RequestType> {
    println!("Message received from {}", addr);
    let size = stream.read_i128().await?;
    println!("Size is {}", size);
    let mut bytes: Vec<u8> = vec![];
    let mut buffer = [0; 1024];
    let mut count = 0;
    loop {
        let n = stream.read(&mut buffer).await?;
        bytes.extend_from_slice(&buffer);
        count += n as i128;
        if count >= size {
            break;
        }
    }
    RequestType::try_from(bytes)
}

async fn run_database_connection(ctx : Arc<zmq::Context>, conn : Arc<Mutex<Connection>>, host : HostUrl) -> Result<()> {

    let tcp_url: TcpString = host.try_into().context("Failed to convert host string to tcp")?;
    let tcp = tokio::net::TcpListener::bind("127.0.0.1:8080").await?;
    loop {
        let (stream, addr) = tcp.accept().await.context("Failed to accept connection")?;
        match handle_connection(stream, addr).await?{
            RequestType::Cancel => break,
            RequestType::NetworkRequest(req) => {
                let response = execute_sql_request(req, &conn).await?;
                
            }
        }
    }
    Ok(())
}

#[tokio::main]
async fn main() -> Result<()> {
    // parse the command line args
    let args = std::env::args().collect::<Vec<String>>();
    if args.len() != 3 {
        return Err(anyhow::anyhow!("Too many arguments provided, example usage: mousqlite_db_runner <host:port> <path/to/db>"));
    }
    // validate the host
    let host_cmd_arg = args.get(1).context("Failed to get host string from cmd line arguments")?;
    let host_string = try_parse_host_url(host_cmd_arg).context("Failed to get host")?;
    
    // validate the file path as provided by the second command line argument
    let database_url = args.get(2).context("Failed to get the database url from command line")?;
    let path = std::path::Path::new(database_url);
    if path.exists().not(){
        return Err(anyhow::anyhow!(format!("File at: {database_url} does not exist")));
    }

    // try to open database connection

    let connection = Arc::new(
        tokio::sync::Mutex::new(
            Connection::open(path).await.context("Failed to open database")?));

    // try to open the zmq dealer socket
    // poll the socket for commands

    let zmq_context = Arc::new(Context::new());
    
    run_database_connection(zmq_context, connection, host_string).await
}