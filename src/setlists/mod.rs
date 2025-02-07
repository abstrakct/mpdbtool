use serde::{Deserialize, Serialize};

// TODO:
// setlist.status should be Enum
// setlist.event_date should maybe be some date type

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Setlists {
    #[serde(rename = "setlist")]
    pub data: Vec<Setlist>,
}

impl Setlists {
    pub fn from_xml(xml: &str) -> Result<Self, serde_xml_rust::Error> {
        serde_xml_rust::from_str(xml)
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Setlist {
    #[serde(rename = "status")]
    pub status: String,
    #[serde(rename = "eventDate")]
    pub event_date: String,
    pub source: Option<String>,
    pub artist: Artist,
    pub venue: Venue,
    pub tour: Option<Tour>,
    pub notes: Option<String>,
    #[serde(rename = "sets")]
    pub sets: Sets,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Artist {
    pub name: String,
    #[serde(rename = "sortName")]
    pub sort_name: Option<String>,
    pub mbid: Option<String>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Venue {
    pub name: String,
    pub city: City,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct City {
    pub name: String,
    pub country: Country,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Country {
    pub name: String,
    pub code: Option<String>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Tour {
    pub name: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Sets {
    pub set: Vec<Set>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Set {
    #[serde(rename = "name")]
    pub name: Option<String>,
    #[serde(rename = "encore")]
    pub encore: Option<String>,
    #[serde(rename = "song")]
    pub songs: Vec<Song>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Song {
    #[serde(rename = "name")]
    pub name: String,
    pub segue: Option<String>,
    #[serde(rename = "cover")]
    pub original_artist: Option<Artist>,
    pub notes: Option<String>,
}
