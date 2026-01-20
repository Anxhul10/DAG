use std::{
    fs::File,
    io::BufReader,
    path::Path,
    error::Error
};
use serde_json::Value;

pub fn read_payload<P: AsRef<Path>>(path: P) -> Result<Value, Box<dyn Error>> {
    // Open file in RO mode with buffer
    let file = File::open(path)?;
    let reader = BufReader::new(file);

    // Read the JSON contents of the file
    let u = serde_json::from_reader(reader)?;

    Ok(u)
}
