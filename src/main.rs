mod setlists;

use setlists::Setlists;

fn main() -> std::io::Result<()> {
    let file = std::fs::read_to_string("master_subset.xml")?;

    // let master: Setlists = serde_xml_rust::from_str(&file).unwrap();
    let master = Setlists::from_xml(&file).unwrap();

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
                if let Some(segue) = song.segue {
                    println!("->");
                }
            }
        }
        println!("--------------------");
    }

    Ok(())
}
