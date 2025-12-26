use serde::Deserialize;

/// An artist associated with a track.
#[derive(Debug, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
#[serde(default)]
#[non_exhaustive]
pub struct Artist {
    /// Shazam internal artist identifier.
    pub id: Option<String>,

    /// Apple Music artist identifier.
    pub adamid: Option<String>,
}
