use log::{debug, error, info};
use serde::Deserialize;
use std::collections::HashSet;

use crate::setlists::*;
use crate::slug::*;

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
pub struct Songtitle {
    id: i32,
    title: String,
    is_default: bool,
}

#[derive(Deserialize, Debug)]
pub struct Mpdb {
    base_url: String,
    pub master: Setlists,
    pub countries: Vec<Country>,
    pub cities: Vec<City>,
    pub venues: Vec<Venue>,
    pub artists: Vec<Artist>,
    pub songtitles: Vec<Songtitle>,
    pub aliases: SongAliases,
}

impl Mpdb {
    pub fn new(base_url: String) -> Mpdb {
        Mpdb {
            base_url,
            aliases: SongAliases::new(),
            master: Setlists::new(),
            countries: vec![],
            cities: vec![],
            venues: vec![],
            artists: vec![],
            songtitles: vec![],
        }
    }

    fn extract_all_unique_country_names(&self) -> HashSet<String> {
        self.master
            .data
            .iter()
            .map(|s| s.venue.city.country.name.clone())
            .collect()
    }

    fn extract_all_unique_artists(&self) -> HashSet<String> {
        self.master
            .data
            .iter()
            .map(|s| s.artist.name.clone())
            .chain(self.master.data.iter().flat_map(|s| {
                s.sets.set.iter().flat_map(|set| {
                    set.songs
                        .as_ref()
                        .map(|songs| songs.iter())
                        .unwrap_or_else(|| [].iter())
                        .filter_map(|song| song.original_artist.as_ref().map(|a| a.name.clone()))
                })
            }))
            .collect()
    }

    fn extract_all_unique_cities(&self) -> HashSet<(String, String)> {
        self.master
            .data
            .iter()
            .map(|s| (s.venue.city.name.clone(), s.venue.city.country.name.clone()))
            .collect()
    }

    fn extract_all_unique_venues(&self) -> HashSet<(String, String, String)> {
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

    fn extract_all_unique_songs(&self) -> HashSet<(String, Option<String>)> {
        self.master
            .data
            .iter()
            .flat_map(|s| {
                s.sets.set.iter().flat_map(|set| {
                    set.songs
                        .as_ref()
                        .map(|songs| songs.iter())
                        .unwrap_or_else(|| [].iter())
                        .map(|song| {
                            (
                                song.name.clone(),
                                song.original_artist.as_ref().map(|a| a.name.clone()),
                            )
                        })
                })
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
        debug!("Getting artist ID for {}", artist_name);
        let id = self
            .artists
            .iter()
            .find(|c| c.name == artist_name)
            .map(|c| c.id);
        debug!("Artist ID: {}", id.unwrap_or(0));
        id
    }

    pub async fn add_all_countries(&self) -> reqwest::Result<Vec<Country>> {
        let countries = self.extract_all_unique_country_names();
        let client = reqwest::Client::new();
        let url = format!("{}/api/countries", self.base_url);

        let existing_countries = client.get(&url).send().await?;
        let existing_countries: Vec<Country> = existing_countries.json().await?;
        let existing_countries: HashSet<String> =
            existing_countries.iter().map(|c| c.name.clone()).collect();

        // println!("Existing countries: {existing_countries:?}");

        for country in countries {
            info!("[ADD?] {country}");

            // Check if country already exists
            if existing_countries.contains(&country) {
                info!("[SKIP] {country} already exists.");
                continue;
            }

            // Country doesn't exist, so add it
            let data = serde_json::json!({
                "name": country,
                "slug": country.slug()
            });
            let res = client.post(&url).json(&data).send().await?;
            if res.status().is_success() {
                info!("[SUCC] {country} added (slug {})", country.slug());
            } else {
                error!("[FAIL] adding country {country} (slug {})", country.slug());
            }
        }

        let existing_countries = client.get(&url).send().await?;
        let existing_countries: Vec<Country> = existing_countries.json().await?;
        Ok(existing_countries)
    }

    pub async fn add_all_cities(&self) -> reqwest::Result<Vec<City>> {
        let cities = self.extract_all_unique_cities();
        let client = reqwest::Client::new();
        let url = format!("{}/api/cities", self.base_url);

        let existing_cities = client.get(&url).send().await?;
        let existing_cities: Vec<City> = existing_cities.json().await?;
        let existing_cities: HashSet<(String, i32)> = existing_cities
            .iter()
            .map(|c| (c.name.clone(), c.country_id))
            .collect();

        // println!("Existing cities: {existing_cities:?}");

        for city in cities {
            info!("[ADD?] city {} in country {}", city.0, city.1);

            if let Some(country_id) = self.get_country_id(&city.1) {
                // Check if city already exists
                if existing_cities.contains(&(city.0.clone(), country_id)) {
                    // TODO: send update request instead of skipping?
                    info!(
                        "[SKIP] city {} in country {} already exists.",
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
                    info!("[SUCC] city {} in country {} added.", city.0, city.1);
                } else {
                    error!("Error adding city: {} in country: {}", city.0, city.1);
                }
            }
        }

        let existing_cities = client.get(&url).send().await?;
        let existing_cities: Vec<City> = existing_cities.json().await?;
        Ok(existing_cities)
    }

    pub async fn add_all_venues(&self) -> reqwest::Result<Vec<Venue>> {
        let venues = self.extract_all_unique_venues();
        let client = reqwest::Client::new();
        let url = format!("{}/api/venues", self.base_url);

        let existing_venues = client.get(&url).send().await?;
        let existing_venues: Vec<Venue> = existing_venues.json().await?;
        let existing_venues: HashSet<(String, i32)> = existing_venues
            .iter()
            .map(|c| (c.name.clone(), c.city_id))
            .collect();

        // println!("Existing venues: {existing_venues:?}");

        for venue in venues {
            info!(
                "[ADD?] venue {} in city {} in country {}",
                venue.0, venue.1, venue.2
            );

            if let Some(city_id) = self.get_city_id(&venue.1, &venue.2) {
                // Check if venue already exists
                if existing_venues.contains(&(venue.0.clone(), city_id)) {
                    info!(
                        "[SKIP] venue {} in city {} in country {} already exists.",
                        venue.0, venue.1, venue.2
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
                    info!(
                        "[SUCC] venue {} in city {} in country {} added (slug {})",
                        venue.0, venue.1, venue.2, slug
                    );
                } else {
                    error!(
                        "[FAIL] adding venue {} in city {} - city id {city_id} - in country {}",
                        venue.0, venue.1, venue.2
                    );
                }
            }
        }

        let existing_venues = client.get(&url).send().await?;
        let existing_venues: Vec<Venue> = existing_venues.json().await?;
        Ok(existing_venues)
    }

    pub async fn add_all_artists(&self) -> reqwest::Result<Vec<Artist>> {
        let artists = self.extract_all_unique_artists();
        let client = reqwest::Client::new();
        let url = format!("{}/api/artists", self.base_url);

        let existing_artists = client.get(&url).send().await?;
        let existing_artists: Vec<Artist> = existing_artists.json().await?;
        let existing_artists: HashSet<String> =
            existing_artists.iter().map(|a| a.name.clone()).collect();

        // Make sure Motorpsycho exists and is the first artist
        let mp = "Motorpsycho";
        if !existing_artists.contains(mp) {
            let data = serde_json::json!({
                "name": mp,
                "slug": mp.to_string().slug()
            });
            let res = client.post(&url).json(&data).send().await?;
            if res.status().is_success() {
                info!("[SUCC] Motorpsycho added");
            } else {
                error!("[FAIL] adding Motorpsycho");
            }
        }

        for artist in artists {
            info!("[ADD?] artist {}", artist);

            // Check if artist already exists
            if existing_artists.contains(&artist) {
                info!("[SKIP] artist {} already exists.", artist);
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
                info!("[SUCC] artist {} added", artist);
            } else {
                error!("[FAIL] adding artist: {}", artist);
            }
        }

        let existing_artists = client.get(&url).send().await?;
        let existing_artists: Vec<Artist> = existing_artists.json().await?;
        Ok(existing_artists)
    }

    pub async fn add_all_songaliases(&self) -> reqwest::Result<()> {
        // let songtitles = self.extract_all_unique_songs();
        let client = reqwest::Client::new();
        let url = format!("{}/api/songtitles", self.base_url);

        let songclient = reqwest::Client::new();
        let songurl = format!("{}/api/songs", self.base_url);

        debug!("Adding songaliases");

        // let existing_songtitles = client.get(&url).send().await?;
        // let existing_songtitles: Vec<Songtitle> = existing_songtitles.json().await?;
        // let existing_songtitles: HashSet<String> = existing_songtitles
        //     .iter()
        //     .map(|s| s.title.clone())
        //     .collect();

        for songwithaliases in self.aliases.songs.clone() {
            // Check if songtitle already exists
            // if existing_songtitles.contains(&songwithaliases.name) {
            //     info!(
            //         "[SKIP] songtitle {} (slug {}) already exists.",
            //         songwithaliases.name,
            //         songwithaliases.name.slug()
            //     );
            //     continue;
            // }

            // songtitle doesn't exist, so add it

            // add a song and get the id
            let songdata = serde_json::json!({
                "artist_id": 1,
            });
            let songres = songclient.post(&songurl).json(&songdata).send().await?;
            let song_json: serde_json::Value = songres.json().await?;
            let song_id = song_json["id"].as_i64().unwrap_or_default();
            info!("[SONG] Created song with ID: {}", song_id);

            // add the default songtitle
            let slug = songwithaliases.name.slug();
            let data = serde_json::json!({
                "title": songwithaliases.name,
                "slug": slug,
                "is_default": true,
                "song_id": song_id,
            });
            let res = client.post(&url).json(&data).send().await?;
            if res.status().is_success() {
                info!(
                    "[SUCC] songtitle {} added, slug {}",
                    songwithaliases.name, slug
                );
            } else {
                error!(
                    "[FAIL] adding songtitle: {}, slug {}",
                    songwithaliases.name, slug
                );
            }

            // add the aliases
            for alias in songwithaliases.aliases {
                let slug = alias.name.slug();
                let data = serde_json::json!({
                    "title": alias.name,
                    "slug": slug,
                    "is_default": false,
                    "song_id": song_id,
                });
                let res = client.post(&url).json(&data).send().await?;
                if res.status().is_success() {
                    info!("[SUCC] alias songtitle {} added, slug {}", alias.name, slug);
                } else {
                    error!(
                        "[FAIL] adding alias songtitle: {}, slug {}",
                        alias.name, slug
                    );
                }
            }
        }
        Ok(())
    }

    pub async fn add_all_songtitles(&self) -> reqwest::Result<Vec<Songtitle>> {
        let songtitles = self.extract_all_unique_songs();
        let client = reqwest::Client::new();
        let url = format!("{}/api/songtitles", self.base_url);

        let songclient = reqwest::Client::new();
        let songurl = format!("{}/api/songs", self.base_url);

        debug!("Songtitles: {songtitles:?}");

        let existing_songtitles = client.get(&url).send().await?;
        let existing_songtitles: Vec<Songtitle> = existing_songtitles.json().await?;
        let existing_songtitles: HashSet<String> = existing_songtitles
            .iter()
            .map(|s| s.title.clone())
            .collect();

        for songtitle in songtitles {
            // Check if songtitle already exists
            if existing_songtitles.contains(&songtitle.0) {
                info!(
                    "[SKIP] songtitle {} (slug {}) already exists.",
                    songtitle.0,
                    songtitle.0.slug()
                );
                continue;
            }

            // songtitle doesn't exist, so add it

            // add a song and get the id
            let artist_id = if songtitle.1.is_some() {
                self.get_artist_id(&songtitle.1.unwrap()).unwrap_or(1)
            } else {
                // It should be impossible that Motorpsycho doesn't exist at this point.
                self.get_artist_id("Motorpsycho").unwrap()
            };
            let songdata = serde_json::json!({
                "artist_id": artist_id,
            });
            let songres = songclient.post(&songurl).json(&songdata).send().await?;
            let song_json: serde_json::Value = songres.json().await?;
            let song_id = song_json["id"].as_i64().unwrap_or_default();
            info!(
                "[SONG] Created song with ID: {}, artist_id: {}",
                song_id, artist_id
            );

            // add the songtitle
            let slug = songtitle.0.slug();
            let data = serde_json::json!({
                "title": songtitle.0,
                "slug": slug,
                "is_default": true,
                "song_id": song_id,
            });
            let res = client.post(&url).json(&data).send().await?;
            if res.status().is_success() {
                info!("[SUCC] songtitle {} added, slug {}", songtitle.0, slug);
            } else {
                error!("[FAIL] adding songtitle: {}, slug {}", songtitle.0, slug);
            }
        }

        let existing_songtitles = client.get(&url).send().await?;
        let existing_songtitles: Vec<Songtitle> = existing_songtitles.json().await?;
        Ok(existing_songtitles)
    }
}
