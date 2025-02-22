use mpdblib::*;
use serde::Deserialize;
use std::collections::HashSet;
// use std::io::Result;

mod tests;

#[derive(Deserialize, Debug)]
struct Country {
    name: String,
    id: i32,
}

#[derive(Deserialize, Debug)]
struct MPDB {
    countries: Vec<Country>,
}

impl MPDB {
    fn new() -> MPDB {
        MPDB { countries: vec![] }
    }
}

const MPDB_BASE_URL: &str = "http://localhost:5150";

#[allow(dead_code)]
fn get_all_countries(master: &Setlists) -> HashSet<String> {
    master
        .data
        .iter()
        .map(|s| s.venue.city.country.name.clone())
        .collect()
}

#[allow(dead_code)]
fn get_all_cities(master: &Setlists) -> HashSet<String> {
    master
        .data
        .iter()
        .map(|s| s.venue.city.name.clone())
        .collect()
}

#[allow(dead_code)]
fn setlists_to_db(master: Setlists) -> reqwest::Result<()> {
    let setlist = master.data[3].clone();
    let x = serde_json::to_string(&setlist).unwrap();
    println!("{}", x);

    //let countries = get_all_countries(&master);
    //println!("{:?}", countries);
    //let cities = get_all_cities(&master);
    //println!("{:?}", cities);

    Ok(())
}

async fn add_all_countries(master: &Setlists) -> reqwest::Result<Vec<Country>> {
    let countries = get_all_countries(master);
    let client = reqwest::Client::new();
    let url = format!("{}/api/countries", MPDB_BASE_URL);

    let existing_countries = client.get(&url).send().await?;
    let existing_countries: Vec<Country> = existing_countries.json().await?;
    let existing_countries: HashSet<String> =
        existing_countries.iter().map(|c| c.name.clone()).collect();

    // println!("Existing countries: {existing_countries:?}");

    for country in countries {
        println!("Adding country: {country}");

        // Check if country already exists
        if existing_countries.contains(&country) {
            println!("Country '{country}' already exists - skipping.");
            continue;
        }

        // Country doesn't exist, so add it
        let data = serde_json::json!({"name": country});
        let res = client.post(&url).json(&data).send().await?;
        if res.status().is_success() {
            println!("Added country:  {country}");
        } else {
            println!("Error adding country: {country}");
        }
    }

    let existing_countries = client.get(&url).send().await?;
    let existing_countries: Vec<Country> = existing_countries.json().await?;
    Ok(existing_countries)
}

#[tokio::main]
async fn main() {
    let mut mpdb: MPDB = MPDB::new();
    let file = std::fs::read_to_string("master_subset.xml").unwrap();
    let master = Setlists::from_xml(&file).unwrap();

    // setlists_to_db(master)?;

    let result = add_all_countries(&master).await;
    match result {
        Ok(c) => {
            println!("Added all countries");
            mpdb.countries = c;
            println!("{:?}", mpdb.countries);
        }
        Err(e) => println!("Error adding countries: {e}"),
    }

    // Ok(())
}

#[allow(dead_code)]
fn dump_setlists(master: Setlists) {
    for setlist in master.data {
        println!(
            "Artist: {} (mbid={})",
            setlist.artist.name,
            setlist.artist.mbid.unwrap_or("no mbid".to_string())
        );
        println!("Event Date: {}", setlist.event_date);
        println!("Status: {}", setlist.status);
        if let Some(source) = setlist.source {
            println!("Source: {}", source);
        }
        println!("Venue: {}", setlist.venue.name);
        println!("City: {}", setlist.venue.city.name);
        println!("Country: {}", setlist.venue.city.country.name);
        if let Some(tour) = setlist.tour {
            println!("Tour: {}", tour.name);
        }
        if let Some(notes) = setlist.notes {
            println!("Notes: {}", notes);
        }

        for set in setlist.sets.set {
            println!("\n*Set*");
            if let Some(name) = set.name {
                println!("-- Set name: {}", name);
            }
            for song in set.songs {
                println!("{}", song.name);
                if song.original_artist.is_some() {
                    println!(
                        "-- COVER!!! Original Artist: {}",
                        song.original_artist.unwrap().name
                    );
                }
                if song.notes.is_some() {
                    println!("Song notes: {}", song.notes.unwrap());
                }
                if song.segue.is_some() {
                    println!("->");
                }
            }
        }
        println!("--------------------");
    }
}
