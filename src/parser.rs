use std::io;
use serde::de::DeserializeOwned;

pub fn parse_str_to_json<T: DeserializeOwned>(json_content : &str) -> Result<T, io::Error> {
    let data_parsed: T =  serde_json::from_str(json_content)      
      .map_err(|_| io::Error::new(io::ErrorKind::InvalidData, "Failed to parse JSON"))?;

    Ok(data_parsed)
}
