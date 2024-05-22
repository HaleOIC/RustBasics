#![allow(dead_code)]

use std::collections::{HashMap, HashSet};
use std::error::Error;
use std::path::Path;

use geoutils::Location;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
struct CSVEntry {
    #[serde(rename = "YEAR")]
    time_period: String,

    #[serde(rename = "STATION")]
    station: String,

    #[serde(rename = "Entries 0600-1000")]
    #[serde(deserialize_with = "csv::invalid_option")]
    entries_morning: Option<i32>,

    #[serde(rename = "Exits 0600-1000")]
    #[serde(deserialize_with = "csv::invalid_option")]
    exits_morning: Option<i32>,

    #[serde(rename = "Entries 1000-1500")]
    #[serde(deserialize_with = "csv::invalid_option")]
    entries_midday: Option<i32>,

    #[serde(rename = "Exits 1000-1500")]
    #[serde(deserialize_with = "csv::invalid_option")]
    exits_midday: Option<i32>,

    #[serde(rename = "Entries 1500-1900")]
    #[serde(deserialize_with = "csv::invalid_option")]
    entries_evening: Option<i32>,

    #[serde(rename = "Exits 1500-1900")]
    #[serde(deserialize_with = "csv::invalid_option")]
    exits_evening: Option<i32>,

    #[serde(rename = "Entries 1900 -0600")]
    #[serde(deserialize_with = "csv::invalid_option")]
    entries_midnight: Option<i32>,

    #[serde(rename = "Exits 1900 -0600")]
    #[serde(deserialize_with = "csv::invalid_option")]
    exits_midnight: Option<i32>,

    #[serde(rename = "Entries 0000-2359")]
    #[serde(deserialize_with = "csv::invalid_option")]
    entries_total: Option<i32>,

    #[serde(rename = "Exits 0000-2359")]
    #[serde(deserialize_with = "csv::invalid_option")]
    exits_total: Option<i32>,

    #[serde(rename = "LAT")]
    latitude: f64,

    #[serde(rename = "LONG")]
    longitude: f64,
}


/// To create a location, run:
///
/// ```rust
/// let berlin = Location::new(52.518611, 13.408056);
/// ```
///
/// then pass two locations into this function for a
/// distance in meters.
fn distance_in_meters(point1: Location, point2: Location) -> f64 {
    point1.distance_to(&point2).unwrap().meters()
}

fn main() -> Result<(), Box<dyn Error>> {
    let path = Path::new("trains.csv");

    let entries: Vec<CSVEntry> = csv::Reader::from_path(&path)?
        .deserialize()
        .collect::<Result<_, _>>()?;

    most_least_stations(&entries);      // task1
    search_station(&entries);           // task2




    Ok(())
}

// Task1: 
// Find the most and least used stations on the NSW network at each time of day
fn most_least_stations(entries: &[CSVEntry]) {
    let mut stations: HashMap<String, i32> = HashMap::new();
    entries.iter() 
        .filter_map(|entry| { 
            entry.entries_total.map(|total| (entry.station.clone(), total))
        })
        .for_each(|(station, total)| {
            stations.entry(station)
                .and_modify(|value| *value = (*value).max(total)) // 使用 *value 来正确修改 HashMap 中的值
                .or_insert(total);
        });
    let most_station = stations.iter()
        .max_by_key(|&(_, total)| total)
        .unwrap();
    let least_station = stations.iter()
        .min_by_key(|&(_, total)| total)
        .unwrap();
    println!("===== Task 1 =====");
    println!("Most used station on the NSW network: {}", most_station.0);
    println!("Least used station on the NSW network: {}\n", least_station.0);
}


// Task 2:
// Allow a user to search for a station, and show it's busiest times of day, and busiest year
fn search_station(entries: &[CSVEntry]) {

}


// Task3:
// Which station has had it's yearly utilisation increase the most in the last 5 years?


// Task4: 
// Which station had the biggest percentage change in use over 2020?


// Task5:
// What is the north-most, south-most, east-most and west-most station?



// Task6:
// Find the two closest stations and the two furthest away from eachother.

// Task7:
// Sort stations by their distance from central, and by their usage. Is the order similar, or different? Are there outliers?


// Task8:
// (hard) A meteor is headed for sydney! It is headed for a train station, but we don't know which one. It will destroy all stations within 2 kilometers of it. For each 4-hour period, make a list of which stations would be best or worst to hit.

// Task9:
// (very hard) Make a graph of an interesting result found above, and post it to /r/dataisbeautiful.