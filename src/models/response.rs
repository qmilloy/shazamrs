use super::Track;
use serde::Deserialize;

/// Top-level response returned by the Shazam discovery API.
///
/// A successful recognition typically contains a `track`,
/// but this is not guaranteed (e.g. unrecognized audio).
#[derive(Debug, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
#[serde(default)]
#[non_exhaustive]
pub struct RecognizeResponse {
    /// Recognized track information, if available.
    pub track: Option<Track>,

    /// Internal Shazam tag identifier.
    pub tagid: Option<String>,

    /// Server-side timestamp of the recognition result.
    pub timestamp: Option<u64>,
}
