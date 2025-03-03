use config::Config;
use log::{debug, error, info};

mod mpdb;
mod setlists;
mod slug;

use mpdb::Mpdb;
use setlists::{Setlists, SongAliases};

mod tests;

const CONFIG_FILE: &str = "mpdbtoolconfig.toml";

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Parse config
    let settings = Config::builder()
        .add_source(config::File::with_name(CONFIG_FILE))
        .build()?;

    let mpdb_base_url = settings.get_string("mpdb_base_url")?;

    // Initialize logger
    // env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info")).init();
    pretty_env_logger::init();

    let mut mpdb: Mpdb = Mpdb::new(mpdb_base_url);
    let file = std::fs::read_to_string("master_subset.xml").unwrap();
    // let file = std::fs::read_to_string("master.xml")?;
    mpdb.master = Setlists::from_xml(&file).map_err(|e| {
        error!("XML parsing error: {}", e);
        e
    })?;

    let alias_file = std::fs::read_to_string("master_aliases.xml").unwrap();
    mpdb.aliases = SongAliases::from_xml(&alias_file).map_err(|e| {
        error!("XML parsing error: {}", e);
        e
    })?;

    debug!("{:?}", mpdb.aliases);

    // setlists_to_db(master)?;

    let result = mpdb.add_all_countries().await;
    match result {
        Ok(c) => {
            info!("Added all countries");
            mpdb.countries = c;
            debug!("{:?}", mpdb.countries);
        }
        Err(e) => error!("Error adding countries: {e}"),
    }

    let result = mpdb.add_all_cities().await;
    match result {
        Ok(c) => {
            info!("Added all cities");
            mpdb.cities = c;
            debug!("{:?}", mpdb.cities);
        }
        Err(e) => error!("Error adding cities: {e}"),
    }

    let result = mpdb.add_all_venues().await;
    match result {
        Ok(c) => {
            info!("Added all venues");
            mpdb.venues = c;
            debug!("{:?}", mpdb.venues);
        }
        Err(e) => error!("Error adding venues: {e}"),
    }

    let result = mpdb.add_all_artists().await;
    match result {
        Ok(c) => {
            info!("Added all artists");
            mpdb.artists = c;
            debug!("{:?}", mpdb.artists);
        }
        Err(e) => error!("Error adding artists: {e}"),
    }

    let result = mpdb.add_all_songaliases().await;
    match result {
        Ok(_) => info!("Added all songaliases"),
        Err(e) => error!("Error adding songaliases: {e}"),
    }

    let result = mpdb.add_all_songtitles().await;
    match result {
        Ok(c) => {
            info!("Added all songtitles");
            mpdb.songtitles = c;
            debug!("{:?}", mpdb.songtitles);
        }
        Err(e) => error!("Error adding songtitles: {e}"),
    }
    Ok(())
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
            if let Some(songs) = set.songs {
                for song in songs {
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
        }
        println!("--------------------");
    }
}
