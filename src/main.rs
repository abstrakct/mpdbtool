mod setlists;

use setlists::Setlists;

fn main() -> std::io::Result<()> {
    let file = std::fs::read_to_string("master.xml")?;

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
        println!("--------------------");
    }

    Ok(())
}
