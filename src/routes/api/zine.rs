use std::path::PathBuf;
use serde::{Deserialize, Serialize};

/// Metadata for a volume of the zine
#[derive(Deserialize, Serialize, PartialEq)]
struct ZineVolume {
    /// Volume number
    id: u8,   
    /// The title of the volume
    title: String,
    /// The date of the volume
    date: String,

    #[serde(rename = "page")]    
    /// The pages belonging to the volume
    pages: Vec<ZinePage>,
}

/// Metadata for a page in a zine volume
#[derive(Deserialize, Serialize, PartialEq)]
struct ZinePage {
    /// The title of the page
    title: String,
    /// The author of the page
    author: String,
    /// The path to the .ans file
    path: PathBuf
}