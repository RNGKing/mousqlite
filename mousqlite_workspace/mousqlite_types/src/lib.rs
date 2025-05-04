use anyhow::Context;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
#[serde(untagged)]
pub enum ColumnData{
    Text(String),
    Integer(i64),
    Real(f64),
    Blob(Vec<u8>),
    Null
}

#[derive(Deserialize, Serialize)]
pub struct DataRow(pub Vec<ColumnData>);

#[derive(Deserialize, Serialize)]
pub struct SqlRequest{
    pub request_id : u64,
    pub request : String
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


#[derive(Deserialize, Serialize)]
pub struct RowData{
    pub row_id : u64,
    pub row_data : Vec<DataRow>
}

#[derive(Deserialize, Serialize)]
pub struct SqlResponse{
    pub request_id : u64,
    pub column_names : Vec<String>,
    pub row_data : Vec<DataRow>
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
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
