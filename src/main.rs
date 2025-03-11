// Internal modules
mod cli;
mod mpdb;
mod setlists;
mod slug;
mod tests;

use cli::*;
use mpdb::Mpdb;
use setlists::{Setlists, SongAliases};

// External crates
use clap::Parser;
use config::Config;
use log::{debug, error, info};

const CONFIG_FILE: &str = "mpdbtoolconfig.toml";

async fn populate_db(
    mpdb_base_url: String,
    master_filename: String,
    aliases_filename: String,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut mpdb: Mpdb = Mpdb::new(mpdb_base_url);
    let file = std::fs::read_to_string(master_filename).unwrap();
    // let file = std::fs::read_to_string("master.xml")?;

    mpdb.master = Setlists::from_xml(&file).map_err(|e| {
        error!("XML parsing error: {}", e);
        e
    })?;

    let alias_file = std::fs::read_to_string(aliases_filename).unwrap();
    mpdb.aliases = SongAliases::from_xml(&alias_file).map_err(|e| {
        error!("XML parsing error: {}", e);
        e
    })?;

    debug!("{:?}", mpdb.aliases);

    info!("Populating countries");
    let result = mpdb.populate_contries().await;
    match result {
        Ok(c) => {
            info!("Added all countries");
            mpdb.countries = c;
            debug!("{:?}", mpdb.countries);
        }
        Err(e) => error!("Error adding countries: {e}"),
    }

    info!("Populating cities");
    let result = mpdb.populate_cities().await;
    match result {
        Ok(c) => {
            info!("Added all cities");
            mpdb.cities = c;
            debug!("{:?}", mpdb.cities);
        }
        Err(e) => error!("Error adding cities: {e}"),
    }

    info!("Populating venues");
    let result = mpdb.populate_venues().await;
    match result {
        Ok(c) => {
            info!("Added all venues");
            mpdb.venues = c;
            debug!("{:?}", mpdb.venues);
        }
        Err(e) => error!("Error adding venues: {e}"),
    }

    info!("Populating artists");
    let result = mpdb.populate_artists().await;
    match result {
        Ok(c) => {
            info!("Added all artists");
            mpdb.artists = c;
            debug!("{:?}", mpdb.artists);
        }
        Err(e) => error!("Error adding artists: {e}"),
    }

    info!("Populating songaliases");
    let result = mpdb.populate_songaliases().await;
    match result {
        Ok(_) => info!("Added all songaliases"),
        Err(e) => error!("Error adding songaliases: {e}"),
    }

    info!("Populating songtitles");
    let result = mpdb.populate_songtitles().await;
    match result {
        Ok(c) => {
            info!("Added all songtitles");
            mpdb.songtitles = c;
            debug!("{:?}", mpdb.songtitles);
        }
        Err(e) => error!("Error adding songtitles: {e}"),
    }

    info!("Populating concerts");
    let result = mpdb.populate_concerts().await;
    match result {
        Ok(c) => {
            info!("Added all concerts");
            mpdb.concerts = c;
            debug!("{:?}", mpdb.concerts);
        }
        Err(e) => error!("Error adding concerts: {e}"),
    }

    Ok(())
}

async fn reset_db(_mpdb_base_url: String) -> Result<(), Box<dyn std::error::Error>> {
    // let mut mpdb: Mpdb = Mpdb::new(mpdb_base_url);
    // mpdb.reset_db().await?;
    todo!()
    // Ok(())
}

async fn xml_to_yml(filename: String) -> Result<(), Box<dyn std::error::Error>> {
    info!("Converting XML to YAML");
    info!("Reading XML file: {}", filename);
    let alias_file = std::fs::read_to_string(filename.clone()).unwrap();
    let aliases = SongAliases::from_xml(&alias_file).map_err(|e| {
        error!("XML parsing error: {}", e);
        e
    })?;

    info!("Converting XML to YAML");
    let yml = aliases.to_yml()?;
    let output_filename = format!("{}.yml", filename.split(".").next().unwrap());
    info!("Writing YAML to file: {}", output_filename);
    std::fs::write(output_filename, yml)?;

    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Parse config
    let settings = Config::builder()
        .add_source(config::File::with_name(CONFIG_FILE))
        .build()?;

    let mpdb_base_url = settings.get_string("mpdb_base_url")?;
    let master_filename = settings.get_string("master_filename")?;
    let aliases_filename = settings.get_string("aliases_filename")?;

    // Parse CLI arguments
    let cli = Cli::parse();

    // Initialize logger
    // env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info")).init();
    pretty_env_logger::init();

    match cli.command {
        Commands::Db { command } => match command {
            DbCommands::Populate => {
                populate_db(mpdb_base_url, master_filename, aliases_filename).await?
            }
            DbCommands::Reset => reset_db(mpdb_base_url).await?,
        },
        Commands::Xml { command } => match command {
            XmlCommands::Convert => xml_to_yml(aliases_filename).await?,
        },
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
