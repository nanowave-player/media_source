use std::time::{SystemTime, UNIX_EPOCH};
use rand::distributions::{Alphanumeric, DistString};
use serde::{Deserialize, Serialize};

const SESSION_KEY_MAX_AGE: u128 = 1000 * 60 * 30; // 30 mins

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MediaSourceSessionKey {
    pub time: u128,
    pub key: String,
    pub expires: u128,
}

impl MediaSourceSessionKey {

    pub fn new() -> Self {
        Self::create(Self::now(), Self::key())
    }

    fn create(time: u128, key: String) -> Self {
        Self {
            time,
            key,
            expires: time + SESSION_KEY_MAX_AGE,
        }
    }

    pub fn now() -> u128 {
        SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis()
    }

    pub fn key() -> String {
        Alphanumeric.sample_string(&mut rand::thread_rng(), 16)
    }

    pub fn extend_validity(&mut self) {
        self.expires = Self::now() + SESSION_KEY_MAX_AGE;
    }

    /*
    pub fn renew(&mut self) {
        self.time = Self::now();
        self.key = Self::key();
        self.expires = self.time + SESSION_MAX_AGE;
    }
*/

    pub fn is_expired(&self) -> bool {
        Self::now() > self.expires
    }

    pub fn to_string(&self) -> String {
        format!("{}.{}", self.time, self.key)
    }


    pub fn parse_string(s: &str) -> Result<Self, &'static str> {
        if s.is_empty() {
            return Err("Empty string");
        }

        let parts: Vec<&str> = s.splitn(2, '.').collect();
        if parts.len() != 2 {
            return Err("No dot separator found");
        }

        let time_part = parts[0];
        let key_part = parts[1];

        let time = time_part.parse::<u128>()
            .map_err(|_| "Invalid u128")?;

        Ok(Self {
            time,
            key: key_part.to_string(),
            expires: time + SESSION_KEY_MAX_AGE,
        })
    }
}