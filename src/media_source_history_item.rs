use std::time::{Duration, SystemTime};
use serde::{Deserialize, Serialize};
use crate::media_source_item::MediaSourceItem;
use crate::media_source_session_key::MediaSourceSessionKey;


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MediaSourceHistoryItem {
    pub item: MediaSourceItem,
    pub session_key: MediaSourceSessionKey,
    pub position: Duration,
    pub date_modified: SystemTime,
}

impl MediaSourceHistoryItem {
    pub fn new(item: MediaSourceItem, session_key: MediaSourceSessionKey, position: Duration, date_modified: SystemTime) -> Self {
        Self {
            item,
            session_key,
            position,
            date_modified
        }
    }
}