use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum MediaType {
    Unspecified = 0,
    Audiobook = 2,
    Music = 4,
}
