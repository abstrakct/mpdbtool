use serde::{Deserialize, Serialize};

// TODO:
// setlist.status should be Enum
// setlist.event_date should maybe be some date type

#[derive(Debug, Deserialize, Serialize)]
pub struct Setlists {
    #[serde(rename = "setlist")]
    pub data: Vec<Setlist>,
}

impl Setlists {
    pub fn from_xml(xml: &str) -> Result<Self, serde_xml_rust::Error> {
        serde_xml_rust::from_str(xml)
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Setlist {
    #[serde(rename = "status")]
    pub status: String,
    #[serde(rename = "eventDate")]
    pub event_date: String,
    pub source: Option<String>,
    pub artist: Artist,
    pub venue: Venue,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Artist {
    pub name: String,
    #[serde(rename = "sortName")]
    pub sort_name: Option<String>,
    pub mbid: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Venue {
    pub name: String,
    pub city: City,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct City {
    pub name: String,
    pub country: Country,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Country {
    pub name: String,
    pub code: Option<String>,
}
