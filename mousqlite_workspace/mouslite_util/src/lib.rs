use pest_derive::Parser;
use pest::Parser;
use anyhow::{Context as _ , Result};

#[derive(Parser)]
#[grammar = "ip_parser.pest"]
pub struct IpParser;

pub fn try_parse_from_str(input : &str) -> Result<String> {
    IpParser::parse(Rule::host_url, input)
        .context(format!("parsing {} failed", input))?;
    Ok(input.to_string())
}

pub fn try_parse_from_string(input : String) -> Result<String> {
    IpParser::parse(Rule::host_url, &input)
        .context(format!("parsing {} failed", input))?;
    Ok(input)
}