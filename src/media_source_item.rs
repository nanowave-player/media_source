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
