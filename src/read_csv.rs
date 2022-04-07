use std::error::Error;
use std::fs::File;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Record {
    pub Id: String,
    pub ActivityDate: String,
    pub TotalSteps: i32,
    pub TotalDistance: f32,
    pub Calories: i32,
}

pub fn parse(file_path: &str) -> Result<Vec<Record>, Box<dyn Error>> {
    let file = File::open(file_path)?;
    let mut rdr = csv::Reader::from_reader(file);
    let mut result = Vec::new();
    for row in rdr.deserialize::<Record>() {
        let record = row?;
        result.push(record);
    }
    Ok(result)
}
