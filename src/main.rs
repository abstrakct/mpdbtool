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

enum FileFormat {
    Xml,
    Yml,
}

impl FileFormat {
    fn extension(&self) -> &'static str {
        match self {
            FileFormat::Xml => "xml",
            FileFormat::Yml => "yml",
        }
    }
}

async fn populate_db(mpdb: &mut Mpdb) -> Result<(), Box<dyn std::error::Error>> {
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

    // TODO: We don't need this anymore, do we?
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

async fn xml_to_yml(alias_filename: String, master_filename: String) -> Result<(), Box<dyn std::error::Error>> {
    info!("Converting XML to YAML");

    // First, the aliases file
    info!("Reading alias file: {}", alias_filename);
    let alias_file = std::fs::read_to_string(alias_filename.clone()).unwrap();
    let aliases = SongAliases::from_xml(&alias_file).map_err(|e| {
        error!("XML parsing error: {}", e);
        e
    })?;

    info!("Converting aliases XML to YAML");
    let yml = aliases.to_yml()?;
    let output_filename = format!("{}.yml", alias_filename.split(".").next().unwrap());
    info!("Writing aliases to YAML file: {}", output_filename);
    std::fs::write(output_filename, yml)?;

    // Then, the master file
    info!("Reading master file: {}", master_filename);
    let master_file = std::fs::read_to_string(master_filename.clone()).unwrap();
    let master = Setlists::from_xml(&master_file).map_err(|e| {
        error!("XML parsing error: {}", e);
        e
    })?;

    info!("Converting master XML to YAML");
    let yml = master.to_yml()?;
    let output_filename = format!("{}.yml", master_filename.split(".").next().unwrap());
    info!("Writing master to YAML file: {}", output_filename);
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
            DbCommands::Populate { xml, yml } => {
                let mut mpdb = Mpdb::new(mpdb_base_url);

                // Determine file format
                let format = match (xml, yml) {
                    (true, false) => FileFormat::Xml,
                    (false, true) => FileFormat::Yml,
                    _ => return Err("Exactly one format (--xml or --yml) must be specified".into()),
                };

                // Load and parse files
                debug!("Loading master file: {}.{}", master_filename, format.extension());
                let master_content = std::fs::read_to_string(format!("{}.{}", master_filename, format.extension()))?;
                debug!("Loading alias file: {}.{}", aliases_filename, format.extension());
                let alias_content = std::fs::read_to_string(format!("{}.{}", aliases_filename, format.extension()))?;

                mpdb.master = match format {
                    FileFormat::Xml => {
                        Setlists::from_xml(&master_content).map_err(|e| format!("XML parse error: {}", e))
                    }
                    FileFormat::Yml => {
                        Setlists::from_yml(&master_content).map_err(|e| format!("YAML parse error: {}", e))
                    }
                }
                .map_err(|e| {
                    error!("Failed to parse master file: {}", e);
                    e
                })?;

                mpdb.aliases = match format {
                    FileFormat::Xml => {
                        SongAliases::from_xml(&alias_content).map_err(|e| format!("XML parse error: {}", e))
                    }
                    FileFormat::Yml => {
                        SongAliases::from_yml(&alias_content).map_err(|e| format!("YAML parse error: {}", e))
                    }
                }
                .map_err(|e| {
                    error!("Failed to parse aliases file: {}", e);
                    e
                })?;

                populate_db(&mut mpdb).await?
            }
            DbCommands::Reset => reset_db(mpdb_base_url).await?,
        },
        Commands::Xml { command } => match command {
            XmlCommands::Convert => xml_to_yml(aliases_filename, master_filename).await?,
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
                        println!("-- COVER!!! Original Artist: {}", song.original_artist.unwrap().name);
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
