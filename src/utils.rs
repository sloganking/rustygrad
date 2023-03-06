use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};

pub struct DataPoint {
    x: f64,
    y: f64,
    label: f64,
}

pub fn read_csv_file(filename: &str) -> Result<Vec<DataPoint>, Box<dyn Error>> {
    let file = File::open(filename)?;
    let reader = BufReader::new(file);

    let mut data_points = vec![];

    for (index, line) in reader.lines().enumerate() {
        let line = line?;

        if index == 0 {
            // Skip the header row
            continue;
        }

        let fields: Vec<&str> = line.split(',').collect();

        let x = fields[0].parse::<f64>()?;
        let y = fields[1].parse::<f64>()?;
        let label = fields[2].parse::<f64>()?;

        let data_point = DataPoint { x, y, label };
        data_points.push(data_point);
    }

    Ok(data_points)
}
