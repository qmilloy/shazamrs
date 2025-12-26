use serde::Deserialize;

/// An action associated with a track or section.
///
/// Actions typically represent external links
/// or playback options.
#[derive(Debug, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
#[serde(default)]
#[non_exhaustive]
pub struct Action {
    /// Human-readable action name.
    pub name: Option<String>,

    /// Action type (e.g. "uri").
    ///
    /// Named `r#type` because `type` is a Rust keyword.
    pub r#type: Option<String>,

    /// Target URI for the action.
    pub uri: Option<String>,
}
