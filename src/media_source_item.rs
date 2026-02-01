use serde::{Deserialize, Serialize};
use crate::media_source_metadata::MediaSourceMetadata;
use crate::media_type::MediaType;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MediaSourceItem {
    pub id: String,
    pub location: String,
    pub title: String,
    pub media_type: MediaType,
    pub metadata: MediaSourceMetadata,
}

impl MediaSourceItem {
    pub fn empty() -> Self {
        Self {
            id: String::from(""),
            location: String::from(""),
            title: String::from(""),
            media_type: MediaType::Unspecified,
            metadata: MediaSourceMetadata::empty()
        }
    }

}