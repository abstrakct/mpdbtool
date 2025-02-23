use mpdblib::*;
use reqwest::get;
use serde::Deserialize;
use std::collections::HashSet;
// use std::io::Result;

mod tests;

trait Slug {
    fn slug(&self) -> String;
}

impl Slug for String {
    /// Converts the string to lowercase, replaces all non-ASCII characters with
    /// hyphens, replaces all spaces with hyphens, and trims leading and trailing
    /// whitespace. Returns the modified string.
    fn slug(&self) -> String {
        // convert to lowercase
        // replace all non-ascii characters with a hyphen
        // replace all spaces with a hyphen
        // trim leading and trailing whitespace
        self.to_lowercase()
            .replace(|c: char| !c.is_ascii(), "-")
            .replace(" ", "-")
            .trim_ascii()
            .to_string()
    }
}

#[derive(Deserialize, Debug)]
struct Country {
    id: i32,
    name: String,
}

#[derive(Deserialize, Debug)]
struct City {
    id: i32,
    name: String,
    country_id: i32,
}

#[derive(Deserialize, Debug)]
struct Venue {
    id: i32,
    name: String,
    city_id: i32,
}

#[derive(Deserialize, Debug)]
struct Mpdb {
    master: Setlists,
    countries: Vec<Country>,
    cities: Vec<City>,
    venues: Vec<Venue>,
}

impl Mpdb {
    fn new() -> Mpdb {
        Mpdb {
            master: Setlists::new(),
            countries: vec![],
            cities: vec![],
            venues: vec![],
        }
    }

    fn get_country_id(&self, country_name: &str) -> Option<i32> {
        self.countries
            .iter()
            .find(|c| c.name == country_name)
            .map(|c| c.id)
    }

    fn get_city_id(&self, city_name: &str, country_name: &str) -> Option<i32> {
        self.cities
            .iter()
            .find(|c| {
                c.name == city_name && c.country_id == self.get_country_id(country_name).unwrap()
            })
            .map(|c| c.id)
    }

    pub async fn add_all_countries(&self) -> reqwest::Result<Vec<Country>> {
        let countries = extract_all_country_names(&self.master);
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

    pub async fn add_all_cities(&self) -> reqwest::Result<Vec<City>> {
        let cities = get_all_cities(&self.master);
        let client = reqwest::Client::new();
        let url = format!("{}/api/cities", MPDB_BASE_URL);

        let existing_cities = client.get(&url).send().await?;
        let existing_cities: Vec<City> = existing_cities.json().await?;
        let existing_cities: HashSet<(String, i32)> = existing_cities
            .iter()
            .map(|c| (c.name.clone(), c.country_id))
            .collect();

        println!("Existing cities: {existing_cities:?}");

        for city in cities {
            println!("Adding city: {} in country: {}", city.0, city.1);

            if let Some(country_id) = self.get_country_id(&city.1) {
                // Check if city already exists
                if existing_cities.contains(&(city.0.clone(), country_id)) {
                    // TODO: send update request instead of skipping?
                    println!(
                        "City '{}' in country '{}' already exists - skipping.",
                        city.0, city.1
                    );
                    continue;
                }

                // City doesn't exist, so add it
                let data = serde_json::json!({"name": city.0, "country_id": country_id});
                let res = client.post(&url).json(&data).send().await?;
                if res.status().is_success() {
                    println!("Added city: {} in country: {}", city.0, city.1);
                } else {
                    println!("Error adding city: {} in country: {}", city.0, city.1);
                }
            }
        }

        let existing_cities = client.get(&url).send().await?;
        let existing_cities: Vec<City> = existing_cities.json().await?;
        Ok(existing_cities)
    }

    pub async fn add_all_venues(&self) -> reqwest::Result<Vec<Venue>> {
        let venues = get_all_venues(&self.master);
        let client = reqwest::Client::new();
        let url = format!("{}/api/venues", MPDB_BASE_URL);

        let existing_venues = client.get(&url).send().await?;
        let existing_venues: Vec<Venue> = existing_venues.json().await?;
        let existing_venues: HashSet<(String, i32)> = existing_venues
            .iter()
            .map(|c| (c.name.clone(), c.city_id))
            .collect();

        println!("Existing venues: {existing_venues:?}");

        for venue in venues {
            println!(
                "Adding venue: {} in city: {} in country: {}",
                venue.0, venue.1, venue.2
            );

            if let Some(city_id) = self.get_city_id(&venue.1, &venue.2) {
                // Check if venue already exists
                if existing_venues.contains(&(venue.0.clone(), city_id)) {
                    println!(
                        "venue '{}' in city '{}' already exists - skipping.",
                        venue.0, venue.1
                    );
                    continue;
                }

                // venue doesn't exist, so add it
                let unique_name = format!("{}-{}", venue.0.slug(), venue.1.slug());
                let data = serde_json::json!({
                    "name": venue.0,
                    "city_id": city_id,
                    "unique_name": unique_name
                });
                let res = client.post(&url).json(&data).send().await?;

                if res.status().is_success() {
                    println!("Added venue: {} in city: {}", venue.0, venue.1);
                } else {
                    println!(
                        "Error adding venue: {} in city: {} - city id {city_id}",
                        venue.0, venue.1
                    );
                }
            }
        }

        let existing_venues = client.get(&url).send().await?;
        let existing_venues: Vec<Venue> = existing_venues.json().await?;
        Ok(existing_venues)
    }
}

const MPDB_BASE_URL: &str = "http://localhost:5150";

#[allow(dead_code)]
fn extract_all_country_names(master: &Setlists) -> HashSet<String> {
    master
        .data
        .iter()
        .map(|s| s.venue.city.country.name.clone())
        .collect()
}

#[allow(dead_code)]
fn get_all_cities(master: &Setlists) -> HashSet<(String, String)> {
    master
        .data
        .iter()
        .map(|s| (s.venue.city.name.clone(), s.venue.city.country.name.clone()))
        .collect()
}

#[allow(dead_code)]
fn get_all_venues(master: &Setlists) -> HashSet<(String, String, String)> {
    master
        .data
        .iter()
        .map(|s| {
            (
                s.venue.name.clone(),
                s.venue.city.name.clone(),
                s.venue.city.country.name.clone(),
            )
        })
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

#[tokio::main]
async fn main() {
    let mut mpdb: Mpdb = Mpdb::new();
    let file = std::fs::read_to_string("master_subset.xml").unwrap();
    mpdb.master = Setlists::from_xml(&file).unwrap();

    // setlists_to_db(master)?;

    let result = mpdb.add_all_countries().await;
    match result {
        Ok(c) => {
            println!("Added all countries");
            mpdb.countries = c;
            println!("{:?}", mpdb.countries);
        }
        Err(e) => println!("Error adding countries: {e}"),
    }

    let result = mpdb.add_all_cities().await;
    match result {
        Ok(c) => {
            println!("Added all cities");
            mpdb.cities = c;
            println!("{:?}", mpdb.cities);
        }
        Err(e) => println!("Error adding cities: {e}"),
    }

    let result = mpdb.add_all_venues().await;
    match result {
        Ok(c) => {
            println!("Added all venues");
            mpdb.venues = c;
            println!("{:?}", mpdb.venues);
        }
        Err(e) => println!("Error adding venues: {e}"),
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
