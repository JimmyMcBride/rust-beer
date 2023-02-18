extern crate csv;
extern crate uuid;

use std::error::Error;
use std::fs::File;
use csv::StringRecord;
use uuid::Uuid;

fn main() -> Result<(), Box<dyn Error>> {
		let mut breweries: Vec<StringRecord> = vec![];
		let mut beers: Vec<StringRecord> = vec![];
    let generated_ids: std::collections::HashSet<String> = std::collections::HashSet::new();

    // Read the breweries dataset into a list of vectors
    let file = File::open("breweries.csv")?;
    let mut rdr = csv::Reader::from_reader(file);
    let mut count = 0;
    for result in rdr.records() {
        let record = result?;
        if count == 0 {
            count += 1;
            let mut row: Vec<String> = record.iter().map(|f| f.to_string()).collect();
            if count == 0 {
                count += 1;
                row.push(record);
            }
					breweries.push(record);
        } else {
            let row = record
                .iter()
                .map(|field| field.to_string())
                .collect::<Vec<String>>();
            breweries.push(row);
        }
    }

    // Read the beers dataset into a list of vectors
    let file = File::open("beers.csv")?;
    let mut rdr = csv::Reader::from_reader(file);
    for result in rdr.records() {
        let record = result?;
        let row = record
            .iter()
            .map(|field| field.to_string())
            .collect::<Vec<String>>();
        beers.push(row);
    }

    // Loop through the breweries
    for brewery in breweries.iter_mut() {
        // Generate a new 16-character id for the current brewery
        let brewery_id = generate_id();
        for beer in beers.iter() {
            let beer_id = generate_id();
            // beer[0] = beer_id;
            if beer[2] == brewery[0] {
                if let Some(last_column) = brewery.last_mut() {
                    if !last_column.is_empty() {
                        last_column.push_str(&format!(", {}", beer[0]));
                    } else {
                        *last_column = beer[0].to_string();
                    }
                }
            }
        }

        // Update the brewery id
        // brewery[0] = brewery_id;
    }

    // Write the updated beers back to the csv file
    let mut file = File::create("beers_updated_column.csv")?;
    let mut writer = csv::Writer::from_writer(&mut file);
    for row in beers {
        writer.write_record(&row)?;
    }

    // Write the updated breweries back to the csv file
    let mut file = File::create("breweries_update_column.csv")?;
    let mut writer = csv::Writer::from_writer(&mut file);
    for row in breweries {
        writer.write_record(&row)?;
    }

    Ok(())
}

fn generate_id() -> String {
		Uuid::new_v4().to_string()[..15].to_string()
}
