use log::{debug, error, info, warn};
use serde::{Deserialize, Serialize};
use std::collections::HashSet;

use crate::setlists::*;
use crate::slug::*;

#[derive(Deserialize, Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Serialize)]
pub struct DbId(i32);

#[derive(Deserialize, Debug, Default, Clone, PartialEq, Eq, Hash, Serialize)]
pub struct Country {
    id: DbId,
    name: String,
}

#[derive(Deserialize, Debug, Default, Clone, PartialEq, Eq, Hash, Serialize)]
pub struct City {
    id: DbId,
    name: String,
    country_id: DbId,
}

#[derive(Deserialize, Debug, Default, Clone, PartialEq, Eq, Hash, Serialize)]
pub struct Venue {
    id: DbId,
    name: String,
    city_id: DbId,
}

#[derive(Deserialize, Debug, Default, Clone, PartialEq, Eq, Hash, Serialize)]
pub struct Artist {
    id: DbId,
    name: String,
}

#[derive(Deserialize, Debug, Default, Clone, PartialEq, Eq, Hash, Serialize)]
pub struct Songtitle {
    id: DbId,
    title: String,
    is_default: bool,
}

#[derive(Deserialize, Debug, Default, Clone, PartialEq, Eq, Hash, Serialize)]
pub struct Concert {
    id: DbId,
    artist_id: DbId,
    date: chrono::NaiveDate,
    disambiguation: Option<String>,
    sort_order: Option<i32>,
    source: Option<String>,
    slug: String,
    venue_id: DbId,
}

#[allow(dead_code)]
impl Concert {
    fn identifier(&self) -> String {
        match &self.disambiguation {
            Some(d) => format!("{}-{}", self.date, d).to_string().slug(),
            None => self.date.to_string().slug(),
        }
    }

    fn identifier_with_prefix(&self, prefix: String) -> String {
        match &self.disambiguation {
            Some(d) => format!("{}-{}-{}", prefix, self.date, d).to_string().slug(),
            None => format!("{}-{}", prefix, self.date).to_string().slug(),
        }
    }

    fn set_date(&mut self, date: String) {
        self.date = chrono::NaiveDate::parse_from_str(&date, "%d-%m-%Y").unwrap();
    }
}

#[derive(Deserialize, Debug)]
pub struct Mpdb {
    // Config
    base_url: String,
    // Raw data
    pub master: Setlists,
    // Parsed and structured data
    pub countries: Vec<Country>,
    pub cities: Vec<City>,
    pub venues: Vec<Venue>,
    pub artists: Vec<Artist>,
    pub songtitles: Vec<Songtitle>,
    pub aliases: SongAliases,
    pub concerts: Vec<Concert>,
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
            concerts: vec![],
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
                        .map(|song| (song.name.clone(), song.original_artist.as_ref().map(|a| a.name.clone())))
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

    fn get_country_id(&self, country_name: &str) -> Option<DbId> {
        self.countries.iter().find(|c| c.name == country_name).map(|c| c.id)
    }

    fn get_city_id(&self, city_name: &str, country_name: &str) -> Option<DbId> {
        self.cities
            .iter()
            .find(|c| c.name == city_name && c.country_id == self.get_country_id(country_name).unwrap())
            .map(|c| c.id)
    }

    fn get_venue_id(&self, venue_name: &str) -> Option<DbId> {
        self.venues.iter().find(|v| v.name == venue_name).map(|v| v.id)
    }

    fn get_artist_id(&self, artist_name: &str) -> Option<DbId> {
        debug!("Getting artist ID for {}", artist_name);
        let id = self.artists.iter().find(|c| c.name == artist_name).map(|c| c.id);
        let x = id.unwrap().0;
        debug!("Artist ID: {}", x);
        id
    }

    fn get_concert_id(&self, concert_slug: &str) -> Option<DbId> {
        self.concerts.iter().find(|c| c.slug == concert_slug).map(|c| c.id)
    }

    pub async fn populate_contries(&self) -> reqwest::Result<Vec<Country>> {
        let countries = self.extract_all_unique_country_names();
        let client = reqwest::Client::new();
        let url = format!("{}/api/countries", self.base_url);

        let existing_countries = client.get(&url).send().await?;
        let existing_countries: Vec<Country> = existing_countries.json().await?;
        let existing_countries: HashSet<String> = existing_countries.iter().map(|c| c.name.clone()).collect();

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

    pub async fn populate_cities(&self) -> reqwest::Result<Vec<City>> {
        let cities = self.extract_all_unique_cities();
        let client = reqwest::Client::new();
        let url = format!("{}/api/cities", self.base_url);

        let existing_cities = client.get(&url).send().await?;
        let existing_cities: Vec<City> = existing_cities.json().await?;
        let existing_cities: HashSet<(String, DbId)> =
            existing_cities.iter().map(|c| (c.name.clone(), c.country_id)).collect();

        // println!("Existing cities: {existing_cities:?}");

        for city in cities {
            info!("[ADD?] city {} in country {}", city.0, city.1);

            if let Some(country_id) = self.get_country_id(&city.1) {
                // Check if city already exists
                if existing_cities.contains(&(city.0.clone(), country_id)) {
                    // TODO: send update request instead of skipping?
                    info!("[SKIP] city {} in country {} already exists.", city.0, city.1);
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

    pub async fn populate_venues(&self) -> reqwest::Result<Vec<Venue>> {
        let venues = self.extract_all_unique_venues();
        let client = reqwest::Client::new();
        let url = format!("{}/api/venues", self.base_url);

        let existing_venues = client.get(&url).send().await?;
        let existing_venues: Vec<Venue> = existing_venues.json().await?;
        let existing_venues: HashSet<(String, DbId)> =
            existing_venues.iter().map(|c| (c.name.clone(), c.city_id)).collect();

        // println!("Existing venues: {existing_venues:?}");

        for venue in venues {
            info!("[ADD?] venue {} in city {} in country {}", venue.0, venue.1, venue.2);

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
                        "[FAIL] adding venue {} in city {} - city id {} - in country {}",
                        venue.0, venue.1, city_id.0, venue.2
                    );
                }
            }
        }

        let existing_venues = client.get(&url).send().await?;
        let existing_venues: Vec<Venue> = existing_venues.json().await?;
        Ok(existing_venues)
    }

    pub async fn populate_artists(&self) -> reqwest::Result<Vec<Artist>> {
        let artists = self.extract_all_unique_artists();
        let client = reqwest::Client::new();
        let url = format!("{}/api/artists", self.base_url);

        // Make sure Motorpsycho exists and is the first artist
        let mp = "Motorpsycho";
        let data = serde_json::json!({
            "name": mp,
            "slug": mp.to_string().slug()
        });
        let res = client.post(&url).json(&data).send().await?;
        if res.status().is_success() {
            info!("[SUCC] Motorpsycho added");
        } else {
            warn!("[FAIL] adding Motorpsycho");
        }

        let existing_artists = client.get(&url).send().await?;
        let existing_artists: Vec<Artist> = existing_artists.json().await?;
        let existing_artists: HashSet<String> = existing_artists.iter().map(|a| a.name.clone()).collect();

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

    pub async fn populate_songaliases(&self) -> reqwest::Result<()> {
        // let songtitles = self.extract_all_unique_songs();
        let client = reqwest::Client::new();
        let url = format!("{}/api/songtitles", self.base_url);

        let songclient = reqwest::Client::new();
        let songurl = format!("{}/api/songs", self.base_url);

        debug!("Adding songaliases");

        for songwithaliases in self.aliases.songs.clone() {
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
                    "[SUCC] songtitle {} added, slug {}, song_id {}",
                    songwithaliases.name, slug, song_id
                );
            } else {
                warn!(
                    "[FAIL] adding songtitle: {}, slug {}, song_id {}",
                    songwithaliases.name, slug, song_id
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
                    info!(
                        "[SUCC] alias songtitle {} added, slug {}, song_id {}",
                        alias.name, slug, song_id
                    );
                } else {
                    warn!(
                        "[FAIL] adding alias songtitle: {}, slug {}, song_id {}",
                        alias.name, slug, song_id
                    );
                }
            }
        }
        Ok(())
    }

    pub async fn populate_songtitles(&self) -> reqwest::Result<Vec<Songtitle>> {
        let songtitles = self.extract_all_unique_songs();
        let client = reqwest::Client::new();
        let url = format!("{}/api/songtitles", self.base_url);

        let songclient = reqwest::Client::new();
        let songurl = format!("{}/api/songs", self.base_url);

        debug!("Songtitles: {songtitles:?}");

        let existing_songtitles = client.get(&url).send().await?;
        let existing_songtitles: Vec<Songtitle> = existing_songtitles.json().await?;
        let existing_songtitles: HashSet<String> = existing_songtitles.iter().map(|s| s.title.clone()).collect();

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
            let artist_id = songtitle
                .1
                .as_ref()
                .and_then(|artist_name| self.get_artist_id(artist_name))
                .unwrap_or_else(|| {
                    self.get_artist_id("Motorpsycho")
                        .expect("Artist Motorpsycho should exist!")
                });

            let songdata = serde_json::json!({
                "artist_id": artist_id,
            });
            let songres = songclient.post(&songurl).json(&songdata).send().await?;
            let song_json: serde_json::Value = songres.json().await?;
            let song_id = song_json["id"].as_i64().unwrap_or_default();
            info!("[SONG] Created song with ID: {}, artist_id: {}", song_id, artist_id.0);

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
                info!(
                    "[SUCC] songtitle {} added, slug {}, song_id {}",
                    songtitle.0, slug, song_id
                );
            } else {
                error!(
                    "[FAIL] adding songtitle: {}, slug {}, song_id {}",
                    songtitle.0, slug, song_id
                );
            }
        }

        let existing_songtitles = client.get(&url).send().await?;
        let existing_songtitles: Vec<Songtitle> = existing_songtitles.json().await?;

        Ok(existing_songtitles)
    }

    pub async fn populate_concerts(&self) -> reqwest::Result<Vec<Concert>> {
        let client = reqwest::Client::new();
        let url = format!("{}/api/concerts", self.base_url);

        let existing_concerts = client.get(&url).send().await?;
        let existing_concerts: Vec<Concert> = existing_concerts.json().await?;
        debug!("Existing concerts: {:?}", existing_concerts);

        for setlist in self.master.data.iter() {
            // Create a concert object
            let artist_id = self.get_artist_id(&setlist.artist.name);
            let venue_id = self.get_venue_id(&setlist.venue.name);
            let mut concert = Concert {
                artist_id: artist_id.unwrap_or_default(),
                date: chrono::NaiveDate::parse_from_str(&setlist.event_date, "%d-%m-%Y").unwrap(),
                venue_id: venue_id.unwrap_or_default(),
                disambiguation: setlist.disambiguation.clone(),
                sort_order: setlist.sort_order,
                source: setlist.source.clone(),
                ..Default::default()
            };

            // Now we can create the slug for the concert
            let slug = concert.identifier_with_prefix(setlist.artist.name.clone());
            concert.slug = slug;

            // Loop through the existing concerts and check if the slug already exists
            // If it does, update the concert
            // If it doesn't, add the concert
            if existing_concerts.iter().any(|c| c.slug == concert.slug) {
                info!("[UPDT] {} already exists - updating", concert.slug);

                concert.id = existing_concerts
                    .iter()
                    .find(|c| c.slug == concert.slug)
                    .map(|c| c.id)
                    .unwrap_or_default();

                let url = format!("{}/api/concerts/{}", self.base_url, concert.id.0);
                let res = client.put(&url).json(&concert).send().await?;
                if res.status().is_success() {
                    info!("[SUCC] {} updated", concert.slug);
                } else {
                    error!("[FAIL] updating concert {}", concert.slug);
                }
            } else {
                info!("[ADD!] {}", concert.slug);
                let res = client.post(&url).json(&concert).send().await?;
                if res.status().is_success() {
                    info!("[SUCC] {} added", concert.slug);
                } else {
                    error!("[FAIL] adding concert {}", concert.slug);
                }
            }
        }

        let existing_concerts = client.get(&url).send().await?;
        let existing_concerts: Vec<Concert> = existing_concerts.json().await?;
        Ok(existing_concerts)
    }
}
