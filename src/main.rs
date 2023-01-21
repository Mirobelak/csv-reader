use std::error::Error;
use csv;

fn main() {
    if let Err(err) = read_from_file("./customers.csv") {
        eprintln!("Error: {}", err);
    }
}

fn read_from_file(path: &str) -> Result<(), Box<dyn Error>> {
    //open the csv file
    let mut rdr = csv::Reader::from_path(path)?;

    //iterate over all the records in the csv
    for result in rdr.records() {
        //get the current record
        let record = result?;
        println!("{:?}", record);
    }

    //return Ok if the function was successful
    Ok(())
}
