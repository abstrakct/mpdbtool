use mpdblib::*;
use serde::Deserialize;
use std::collections::HashSet;

pub use crate::slug::*;

#[derive(Deserialize, Debug)]
pub struct Country {
    id: i32,
    name: String,
}

#[derive(Deserialize, Debug)]
pub struct City {
    id: i32,
    name: String,
    country_id: i32,
}

#[derive(Deserialize, Debug)]
pub struct Venue {
    id: i32,
    name: String,
    city_id: i32,
}

#[derive(Deserialize, Debug)]
pub struct Artist {
    id: i32,
    name: String,
}

#[derive(Deserialize, Debug)]
pub struct Mpdb {
    base_url: String,
    pub master: Setlists,
    pub countries: Vec<Country>,
    pub cities: Vec<City>,
    pub venues: Vec<Venue>,
    pub artists: Vec<Artist>,
}

impl Mpdb {
    pub fn new(base_url: String) -> Mpdb {
        Mpdb {
            base_url,
            master: Setlists::new(),
            countries: vec![],
            cities: vec![],
            venues: vec![],
            artists: vec![],
        }
    }

    fn extract_all_country_names(&self) -> HashSet<String> {
        self.master
            .data
            .iter()
            .map(|s| s.venue.city.country.name.clone())
            .collect()
    }

    fn get_all_artists(&self) -> HashSet<String> {
        self.master
            .data
            .iter()
            .map(|s| s.artist.name.clone())
            .chain(self.master.data.iter().flat_map(|s| {
                s.sets.set.iter().flat_map(|set| {
                    set.songs
                        .iter()
                        .filter_map(|song| song.original_artist.as_ref().map(|a| a.name.clone()))
                })
            }))
            .collect()
    }

    fn get_all_cities(&self) -> HashSet<(String, String)> {
        self.master
            .data
            .iter()
            .map(|s| (s.venue.city.name.clone(), s.venue.city.country.name.clone()))
            .collect()
    }

    fn get_all_venues(&self) -> HashSet<(String, String, String)> {
        self.master
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

    fn get_artist_id(&self, artist_name: &str) -> Option<i32> {
        self.artists
            .iter()
            .find(|c| c.name == artist_name)
            .map(|c| c.id)
    }

    pub async fn add_all_countries(&self) -> reqwest::Result<Vec<Country>> {
        let countries = self.extract_all_country_names();
        let client = reqwest::Client::new();
        let url = format!("{}/api/countries", self.base_url);

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
            let data = serde_json::json!({
                "name": country,
                "slug": country.slug()
            });
            let res = client.post(&url).json(&data).send().await?;
            if res.status().is_success() {
                println!("Added country:  {country} (slug {})", country.slug());
            } else {
                println!("Error adding country: {country} (slug {})", country.slug());
            }
        }

        let existing_countries = client.get(&url).send().await?;
        let existing_countries: Vec<Country> = existing_countries.json().await?;
        Ok(existing_countries)
    }

    pub async fn add_all_cities(&self) -> reqwest::Result<Vec<City>> {
        let cities = self.get_all_cities();
        let client = reqwest::Client::new();
        let url = format!("{}/api/cities", self.base_url);

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
                let slug = format!("{}-{}", city.0.slug(), city.1.slug());
                let data = serde_json::json!({
                    "name": city.0,
                    "country_id": country_id,
                    "slug": slug
                });
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
        let venues = self.get_all_venues();
        let client = reqwest::Client::new();
        let url = format!("{}/api/venues", self.base_url);

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
                let slug = format!("{}-{}-{}", venue.0.slug(), venue.1.slug(), venue.2.slug());
                let data = serde_json::json!({
                    "name": venue.0,
                    "city_id": city_id,
                    "unique_name": unique_name,
                    "slug": slug
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

    pub async fn add_all_artists(&self) -> reqwest::Result<Vec<Artist>> {
        let artists = self.get_all_artists();
        let client = reqwest::Client::new();
        let url = format!("{}/api/artists", self.base_url);

        let existing_artists = client.get(&url).send().await?;
        let existing_artists: Vec<Artist> = existing_artists.json().await?;
        let existing_artists: HashSet<String> =
            existing_artists.iter().map(|a| a.name.clone()).collect();

        println!("Existing artists: {existing_artists:?}");

        for artist in artists {
            println!("Adding artist: {}", artist);

            // Check if artist already exists
            if existing_artists.contains(&artist) {
                println!("artist '{}' already exists - skipping.", artist);
                continue;
            }

            // artist doesn't exist, so add it
            let slug = artist.slug();
            let data = serde_json::json!({
                "name": artist,
                "slug": slug
            });
            let res = client.post(&url).json(&data).send().await?;
            if res.status().is_success() {
                println!("Added artist: {}", artist);
            } else {
                println!("Error adding artist: {}", artist);
            }
        }

        Ok(Vec::new())
    }
}
