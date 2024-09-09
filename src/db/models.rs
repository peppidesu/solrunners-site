use couch_rs::{types::document::DocumentId, CouchDocument};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, CouchDocument)]
pub struct ZineVolume {
    pub _id: DocumentId,
    
    pub title: String,
    pub release_date: String,
    pub pages: Vec<DocumentId>,
}

#[derive(Serialize, Deserialize, CouchDocument)]
pub struct ZinePage {
    pub _id: DocumentId,

    pub volume: DocumentId,
    pub title: String,
    pub author: String,
    pub path: String,
}

