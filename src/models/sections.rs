use super::{Action, Metadata};
use serde::Deserialize;

/// A content section within a track.
///
/// Sections may represent lyrics, related links,
/// videos, or other metadata blocks.
#[derive(Debug, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
#[serde(default)]
#[non_exhaustive]
pub struct Section {
    /// Section type (e.g. "LYRICS", "VIDEO").
    ///
    /// Named `section_type` because `type` is a Rust keyword.
    #[serde(rename = "type")]
    pub section_type: Option<String>,

    /// Display name for the section tab.
    pub tabname: Option<String>,

    /// Metadata entries within the section.
    pub metadata: Option<Vec<Metadata>>,

    /// Actions associated with this section.
    pub actions: Option<Vec<Action>>,
}
