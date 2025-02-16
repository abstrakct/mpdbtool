use mpdblib::*;
use std::collections::HashSet;

mod tests;

// Possible flow:
// Make vector of all unique counties, add each
// same with cities etc
//

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

fn setlists_to_db(master: Setlists) -> std::io::Result<()> {
    let setlist = master.data[3].clone();
    let x = serde_json::to_string(&setlist).unwrap();
    println!("{}", x);

    //let countries = get_all_countries(&master);
    //println!("{:?}", countries);
    //let cities = get_all_cities(&master);
    //println!("{:?}", cities);

    Ok(())
}

fn main() -> std::io::Result<()> {
    let file = std::fs::read_to_string("master_subset.xml")?;
    let master = Setlists::from_xml(&file).unwrap();

    setlists_to_db(master)?;

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
