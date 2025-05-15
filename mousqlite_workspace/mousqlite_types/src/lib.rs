use std::fmt::Display;
use anyhow::Context;
use serde::{Deserialize, Serialize};
use bincode::{config, Decode, Encode};

#[derive(Deserialize, Serialize,Encode, Decode, Clone)]
#[serde(untagged)]
pub enum ColumnData{
    Text(String),
    Integer(i64),
    Real(f64),
    Blob(Vec<u8>),
    Null
}

#[derive(Deserialize, Serialize,Encode, Decode, Clone)]
pub struct DataRow(pub Vec<ColumnData>);

#[derive(Deserialize, Serialize, Encode, Decode, Clone, Debug)]
pub enum RequestType{
    Cancel,
    NetworkRequest(SqlRequest)
}



#[derive(Deserialize, Serialize, Encode, Decode, Clone, Debug)]
pub struct SqlRequest{
    pub request_id : u64,
    pub request : String
}

impl TryInto<Vec<u8>> for RequestType{
    type Error = anyhow::Error;
    fn try_into(self) -> Result<Vec<u8>, Self::Error>{
        let config = config::standard();
        bincode::encode_to_vec(self, config).context("Failed to convert value to Vec<u8>")
    }
}

impl TryFrom<Vec<u8>> for RequestType{
    type Error = anyhow::Error;
    fn try_from(value: Vec<u8>) -> Result<Self, Self::Error> {
        let config = config::standard();
        let (output, _) = bincode::decode_from_slice::<RequestType, bincode::config::Configuration>(&value, config)?;
        Ok(output)
    }
}

impl TryFrom<String> for SqlRequest {
    type Error = anyhow::Error;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        serde_json::from_str::<SqlRequest>(&value)
            .context(format!("Failed to parse : {value}"))
    }
}

impl TryFrom<&str> for SqlRequest{
    type Error = anyhow::Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        serde_json::from_str(value)
            .context(format!("Failed to parse : {value}"))
    }
}

impl TryInto<String> for SqlRequest {
    type Error = anyhow::Error;

    fn try_into(self) -> Result<String, Self::Error> {
        serde_json::to_string(&self)
            .context("Failed to convert request type to json string")
    }
}

impl Default for SqlRequest{
    fn default() -> Self {
        SqlRequest{
            request_id : 0,
            request : String::new()
        }
    }
}

#[derive(Deserialize, Serialize)]
pub struct RowData{
    pub row_id : u64,
    pub row_data : Vec<DataRow>
}

#[derive(Deserialize, Serialize, Encode, Decode, Clone)]
pub struct SqlResponse{
    pub request_id : u64,
    pub column_names : Vec<String>,
    pub row_data : Vec<DataRow>
}

impl TryInto<Vec<u8>> for SqlResponse {
    type Error = anyhow::Error;
    fn try_into(self) -> Result<Vec<u8>, Self::Error> {
        let config = config::standard();
        bincode::encode_to_vec(self, config).context("Failed to convert SqlResponse to Vec<u8>")
    }
}

impl TryFrom<Vec<u8>> for SqlResponse {
    type Error = anyhow::Error;
    fn try_from(value: Vec<u8>) -> Result<Self, Self::Error> {
        let config = config::standard();
        let (output, _) =bincode::decode_from_slice::<SqlResponse, bincode::config::Configuration>(&value, config).context("Failed to convert SqlResponse to Vec<u8>".to_string())?;
        Ok(output)
    }
}

impl Default for SqlResponse{
    fn default() -> Self {
        Self { request_id: Default::default(), column_names: Default::default(), row_data: Default::default() }
    }
}

impl SqlResponse {
    pub fn into_zmq_response(self) -> anyhow::Result<String>{
        serde_json::to_string(&self)
            .context("Failed to convert the sql response into a zmq response")
    }
}

pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {
    use crate::RequestType::Cancel;
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    #[test]
    fn test_output(){
        let test = Cancel;
        println!("{}" , serde_json::to_string(&test).unwrap());
        let test_two = RequestType::NetworkRequest(SqlRequest::default());
        println!("{}", serde_json::to_string(&test_two).unwrap());
    }


}
