use serde::{Deserialize, Serialize};
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct SongAliases {
    #[serde(rename = "song")]
    pub songs: Vec<SongWithAliases>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct SongWithAliases {
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "alias")]
    pub aliases: Vec<Alias>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Alias {
    #[serde(rename = "name")]
    pub name: String,
}

impl Default for SongAliases {
    fn default() -> Self {
        Self::new()
    }
}

#[allow(dead_code)]
impl SongAliases {
    /// Creates a new empty SongAliases struct
    ///
    /// # Returns
    /// * `SongAliases` - A new SongAliases struct with an empty vector of songs
    pub fn new() -> Self {
        SongAliases { songs: Vec::new() }
    }

    /// Parses a SongAliases struct from an XML string
    ///
    /// # Arguments
    /// * `xml` - A string containing XML data
    ///
    /// # Returns
    /// * `Result<Self, serde_xml_rust::Error>` - The parsed SongAliases on success, or a deserialization error
    pub fn from_xml(xml: &str) -> Result<Self, serde_xml_rust::Error> {
        serde_xml_rust::from_str(xml)
    }

    /// Parses a SongAliases struct from a YAML string
    ///
    /// # Arguments
    /// * `yml` - A string containing YAML data
    ///
    /// # Returns
    /// * `Result<Self, serde_yml::Error>` - The parsed SongAliases on success, or a deserialization error
    pub fn from_yml(yml: &str) -> Result<Self, serde_yml::Error> {
        serde_yml::from_str(yml)
    }

    /// Converts the SongAliases struct to a YAML string
    ///
    /// # Returns
    /// * `Result<String, serde_yml::Error>` - The YAML string on success, or a serialization error
    pub fn to_yml(&self) -> Result<String, serde_yml::Error> {
        serde_yml::to_string(self)
    }
}

// TODO:
// setlist.status should be Enum
// setlist.event_date should maybe be some date type

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Setlists {
    #[serde(rename = "setlist")]
    pub data: Vec<Setlist>,
}

impl Default for Setlists {
    fn default() -> Self {
        Self::new()
    }
}

#[allow(dead_code)]
impl Setlists {
    /// Creates a new empty Setlists struct
    ///
    /// # Returns
    /// * `Setlists` - A new Setlists struct with an empty vector of setlists
    pub fn new() -> Self {
        Setlists { data: Vec::new() }
    }

    /// Parses a Setlists struct from an XML string
    ///
    /// # Arguments
    /// * `xml` - A string containing XML data
    ///
    /// # Returns
    /// * `Result<Self, serde_xml_rust::Error>` - The parsed Setlists on success, or a deserialization error
    pub fn from_xml(xml: &str) -> Result<Self, serde_xml_rust::Error> {
        serde_xml_rust::from_str(xml)
    }

    /// Parses a Setlists struct from a YAML string
    ///
    /// # Arguments
    /// * `yml` - A string containing YAML data
    ///
    /// # Returns
    /// * `Result<Self, serde_xml_rust::Error>` - The parsed Setlists on success, or a deserialization error
    pub fn from_yml(yml: &str) -> Result<Self, serde_yml::Error> {
        serde_yml::from_str(yml)
    }

    /// Converts the Setlists struct to a YAML string
    ///
    /// # Returns
    /// * `Result<String, serde_yml::Error>` - The YAML string on success, or a serialization error
    pub fn to_yml(&self) -> Result<String, serde_yml::Error> {
        serde_yml::to_string(self)
    }
}

/// Represents a setlist, which is a collection of songs played by an artist at a specific event.
///
/// A setlist includes information about the event, such as the date and venue, as well as the songs played.
///
/// # Fields
///
/// * `status`: The status of the setlist (e.g. "confirmed", "unconfirmed", etc.)
/// * `event_date`: The date of the event
/// * `disambiguation`: Optional disambiguation information for the event
/// * `sort_order`: Optional sort order for the setlist
/// * `source`: Optional source information for the setlist
/// * `artist`: The artist who played the setlist
/// * `venue`: The venue where the setlist was played
/// * `tour`: Optional tour information for the setlist
/// * `notes`: Optional notes about the setlist
/// * `sets`: The sets played during the event
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Setlist {
    #[serde(rename = "status")]
    pub status: String,
    #[serde(rename = "eventDate")]
    pub event_date: String,
    #[serde(rename = "disambiguation")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disambiguation: Option<String>,
    #[serde(rename = "sortOrder")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_order: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<String>,
    pub artist: Artist,
    pub venue: Venue,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tour: Option<Tour>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notes: Option<String>,
    #[serde(rename = "sets")]
    pub sets: Sets,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Artist {
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "sortName")]
    pub sort_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
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
    #[serde(skip_serializing_if = "Option::is_none")]
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
    pub songs: Option<Vec<Song>>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Song {
    #[serde(rename = "name")]
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub segue: Option<bool>,
    #[serde(rename = "cover")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub original_artist: Option<Artist>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notes: Option<String>,
    // #[serde(rename = "aliasFor")]
    // pub alias_for: Option<String>,
}

impl Default for Song {
    fn default() -> Self {
        Self {
            name: "".to_string(),
            segue: Some(false),
            original_artist: None,
            notes: None,
            // alias_for: None,
        }
    }
}
