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

#[cfg(test)]
mod tests {
    use crate::models::*;

    #[test]
    fn deserializes_full_recognized_response() {
        let json = r#"{
            "track": {
                "key": "12345",
                "title": "Song Title",
                "subtitle": "Artist Name",
                "url": "https://www.shazam.com/track/12345",
                "artists": [{"id": "111", "adamid": "222"}],
                "images": {
                    "background": "https://example.com/bg.jpg",
                    "coverart": "https://example.com/cover.jpg",
                    "coverarthq": "https://example.com/cover_hq.jpg"
                },
                "sections": [
                    {
                        "type": "SONG",
                        "tabname": "Song",
                        "metadata": [{"title": "Album", "text": "Some Album"}],
                        "actions": [{"name": "apple", "type": "uri", "uri": "https://music.apple.com/x"}]
                    }
                ],
                "actions": [{"name": "apple", "type": "uri", "uri": "https://music.apple.com/x"}],
                "genres": {"primary": "Pop"},
                "metadata": [{"title": "Label", "text": "Some Label"}]
            },
            "tagid": "ABCDEF",
            "timestamp": 1700000000
        }"#;

        let response: RecognizeResponse = serde_json::from_str(json).unwrap();

        let track = response.track.expect("track should be present");
        assert_eq!(track.title.as_deref(), Some("Song Title"));
        assert_eq!(track.subtitle.as_deref(), Some("Artist Name"));
        assert_eq!(track.artists.unwrap()[0].id.as_deref(), Some("111"));
        assert_eq!(
            track.images.unwrap().coverart.as_deref(),
            Some("https://example.com/cover.jpg")
        );
        assert_eq!(
            track.sections.unwrap()[0].section_type.as_deref(),
            Some("SONG")
        );
        assert_eq!(track.genres.unwrap().primary.as_deref(), Some("Pop"));
        assert_eq!(response.tagid.as_deref(), Some("ABCDEF"));
        assert_eq!(response.timestamp, Some(1700000000));
    }

    #[test]
    fn deserializes_unrecognized_audio_response() {
        // Shazam returns a body with no `track` key when audio isn't recognized.
        let json = r#"{"timestamp": 1700000000}"#;

        let response: RecognizeResponse = serde_json::from_str(json).unwrap();

        assert!(response.track.is_none());
        assert_eq!(response.timestamp, Some(1700000000));
    }

    #[test]
    fn deserializes_empty_object() {
        let response: RecognizeResponse = serde_json::from_str("{}").unwrap();

        assert!(response.track.is_none());
        assert!(response.tagid.is_none());
        assert!(response.timestamp.is_none());
    }

    #[test]
    fn track_tolerates_missing_optional_fields() {
        let json = r#"{"track": {"title": "Only A Title"}}"#;

        let response: RecognizeResponse = serde_json::from_str(json).unwrap();

        let track = response.track.expect("track should be present");
        assert_eq!(track.title.as_deref(), Some("Only A Title"));
        assert!(track.subtitle.is_none());
        assert!(track.artists.is_none());
        assert!(track.images.is_none());
        assert!(track.sections.is_none());
    }
}
