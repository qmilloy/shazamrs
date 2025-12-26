use super::{Action, Artist, Images, Metadata, Section};
use serde::Deserialize;

/// A music track recognized by Shazam.
///
/// This is the primary data structure returned when
/// audio is successfully identified.
#[derive(Debug, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
#[serde(default)]
#[non_exhaustive]
pub struct Track {
    /// Internal Shazam track key.
    pub key: Option<String>,

    /// Track title.
    pub title: Option<String>,

    /// Artist or group name.
    pub subtitle: Option<String>,

    /// Canonical Shazam web URL for the track.
    pub url: Option<String>,

    /// Artists associated with the track.
    pub artists: Option<Vec<Artist>>,

    /// Artwork and image assets.
    pub images: Option<Images>,

    /// Content sections such as lyrics, links, or videos.
    pub sections: Option<Vec<Section>>,

    /// Action buttons (e.g. Apple Music, Spotify).
    pub actions: Option<Vec<Action>>,

    /// Genre information.
    pub genres: Option<Genres>,

    /// Additional metadata entries (album, label, etc).
    pub metadata: Option<Vec<Metadata>>,
}

/// Genre classification for a track.
#[derive(Debug, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
#[serde(default)]
#[non_exhaustive]
pub struct Genres {
    /// Primary genre name.
    pub primary: Option<String>,
}
