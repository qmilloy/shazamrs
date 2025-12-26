use serde::Deserialize;

/// Image assets associated with a track.
///
/// URLs typically point to CDN-hosted images.
#[derive(Debug, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
#[serde(default)]
#[non_exhaustive]
pub struct Images {
    /// Background image URL.
    pub background: Option<String>,

    /// Standard-resolution cover art.
    pub coverart: Option<String>,

    /// High-resolution cover art.
    pub coverarthq: Option<String>,
}
