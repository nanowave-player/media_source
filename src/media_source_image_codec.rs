use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum MediaSourceImageCodec {
    Unknown,
    Jpeg,
    Png,
    Tiff,
    Bmp,
    Gif,
    WebP,
}
