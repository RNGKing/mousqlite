use poem_openapi::{ApiRequest, ApiResponse, Object, Tags, Union};
use poem_openapi::payload::PlainText;

#[derive(Tags)]
pub enum ApiTags{
    Query,
}

#[derive(Union)]
pub enum ColumnData{
    Text(String),
    Integer(i64),
    Real(f64),
    Blob(Vec<u8>),
    Null(Option<i32>)
}

#[derive(Object)]
pub struct Row{
    data : Vec<ColumnData>
}

impl Row{
    pub fn new (data : Vec<ColumnData>)->Self{
        Self{data}
    }
}

#[derive(Object)]
pub struct SqlResponse{
    columns : Vec<String>,
    row_data : Vec<Row>
}

impl SqlResponse{
    pub fn new (columns : Vec<String>, row_data : Vec<Row>)->Self{
        Self{columns, row_data}
    }
}

#[derive(ApiResponse)]
pub enum QueryResponse{
    #[oai(status = 200)]
    Ok,
    #[oai(status = 400)]
    QueryError(PlainText<String>)
}


#[derive(Object)]
pub struct SqlRequest{
    pub query: String
}