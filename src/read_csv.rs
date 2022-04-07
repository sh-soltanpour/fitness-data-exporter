use std::error::Error;
use std::fs::File;
use std::io;
use std::process;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Record {
    pub Id: String,
    pub ActivityDate: String,
    pub TotalSteps: i32,
    pub TotalDistance: f32,
    pub Calories: i32,
}

pub fn parse(file_path: &str) -> Result<(Vec<Record>), Box<dyn Error>> {
    // Build the CSV reader and iterate over each record.
    // let file_path = "src/dailyActivity_merged.csv";

    let file = File::open(file_path)?;
    let mut rdr = csv::Reader::from_reader(file);
    let mut result = Vec::new();
    let x = rdr.deserialize::<Record>();
    for row in rdr.deserialize::<Record>() {
        let record = row?;
        // println!("{}", record.Miles);
        result.push(record);
        // track_fitness_data(
        //     &record.ActivityDate.to_string(),
        //     &record.TotalSteps.to_string(),
        //     &record.TotalDistance.to_string(),
        //     &record.Calories.to_string(),
        // )
    }
    Ok(result)
}
