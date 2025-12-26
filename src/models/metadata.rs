use serde::Deserialize;

/// A key-value metadata entry.
///
/// Used throughout the response to represent
/// additional descriptive information.
#[derive(Debug, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
#[serde(default)]
#[non_exhaustive]
pub struct Metadata {
    /// Metadata field name.
    pub title: Option<String>,

    /// Metadata field value.
    pub text: Option<String>,
}
