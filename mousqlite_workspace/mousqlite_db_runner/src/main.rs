use std::ops::Not;

use anyhow::{Context as _, Result};
use mousqlite_types::{ColumnData, Row, SqlRequest, SqlResponse};
use pest::Parser;
use pest_derive::Parser;
use rusqlite::types::ValueRef;
use tokio_rusqlite::Connection;
use std::sync::Arc;
use poem::{ EndpointExt, Route, Server};
use poem::error::InternalServerError;
use poem::web::Data;
use tokio::sync::Mutex;
use poem_openapi::OpenApi;
use poem_openapi::payload::Json;
use mousqlite_types::ApiTags;

#[derive(Parser)]
#[grammar = "arg_parse_validation.pest"]
struct ArgParser;

struct HostUrl(String);
struct TcpString(String);

struct Api;

#[OpenApi]
impl Api {
    #[oai(path="/query",method="post", tag="ApiTags::Query")]
    async fn query(&self,
                   connection : Data<&Arc<Mutex<Connection>>>,
                   sql_request: Json<SqlRequest>
    ) -> poem::Result<Json<SqlResponse>> {
        let output = execute_sql_request(sql_request.0, &connection.0)
            .await?;
        Ok(Json(output))
    }
}

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
        ValueRef::Null => Ok(ColumnData::Null(None)),
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

fn get_row_data( row : &rusqlite::Row, column_count : usize) -> std::result::Result<Vec<ColumnData>, rusqlite::Error> {
    let mut row_data = vec![];
    for column in 0..column_count {
        let data = row.get_ref(column)?;
        let converted = match convert_valueref_to_columndata(data) {
            Ok(col_data) => col_data,
            Err(_) => return Err(rusqlite::Error::InvalidQuery),
        };
        row_data.push(converted);
    }
    Ok(row_data)
}

async fn execute_sql_request(req : SqlRequest, conn : &Arc<Mutex<Connection>>) -> poem::Result<SqlResponse>{
    let connection = conn.lock().await;
    let output = connection.call(move | conn| {
        let mut stmt = conn.prepare(&req.query)?;
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
                    output_vec.push(Row::new(row_data));
                },
                Err(_) => {return Err(tokio_rusqlite::Error::ConnectionClosed); }
            }
        }
        Ok(SqlResponse::new(column_metadata, output_vec))
    }).await;
    match output {
        Ok(response) => Ok(response),
        Err(err) => Err(InternalServerError(err)),
    }
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
    //let host_string = try_parse_host_url(host_cmd_arg).context("Failed to get host")?;
    
    // validate the file path as provided by the second command line argument
    let database_url = args.get(2).context("Failed to get the database url from command line")?;
    let path = std::path::Path::new(database_url);
    if path.exists().not(){
        return Err(anyhow::anyhow!(format!("File at: {database_url} does not exist")));
    }

    let db_path = std::path::Path::new("/Users/test/projects/rust_projects/mousqlite/mousqlite_workspace/database/test.db");
    database_helpers::run_helper(db_path)?;
    
    // try to open database connection

    let connection = Arc::new(
        tokio::sync::Mutex::new(
            Connection::open(path).await.context("Failed to open database")?));

    let api_service =
        poem_openapi::OpenApiService::new(Api, "Mousqlite_Service", "1.0").server("http://localhost:3000");
    let ui = api_service.swagger_ui();
    let app = Route::new()
        .nest("/", api_service)
        .nest("docs", ui)
        .data(connection.clone());
    Server::new(poem::listener::TcpListener::bind("127.0.0.1:3000"))
        .run(app)
        .await
        .context("Failed to run server")
}