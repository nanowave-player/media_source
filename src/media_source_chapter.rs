use crate::duration_millis;
use serde::{Deserialize, Serialize};
use std::time::Duration;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MediaSourceChapter {
    pub name: String,
    #[serde(with = "duration_millis")]
    pub start: Duration,
    #[serde(with = "duration_millis")]
    pub duration: Duration,
}

impl MediaSourceChapter {
    pub fn new(name: String, start: Duration, duration: Duration) -> Self {
        Self {
            name,
            start,
            duration,
        }
    }

    pub fn end(&self) -> Duration {
        self.start + self.duration
    }
}
