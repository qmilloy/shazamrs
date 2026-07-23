//! Response models for the Shazam discovery API.
//!
//! These structs are designed to:
//! - Match Shazam’s JSON responses
//! - Be tolerant of missing or optional fields
//! - Remain forward-compatible as the API evolves
//!
//! All fields are optional unless guaranteed by the backend.

/// Actions associated with a track or section.
pub mod actions;
/// Artists associated with a track.
pub mod artist;
/// Image assets associated with a track.
pub mod images;
/// Key-value metadata entries.
pub mod metadata;
/// Top-level response returned by the Shazam discovery API.
pub mod response;
/// Content sections within a track.
pub mod sections;
/// Music tracks recognized by Shazam.
pub mod track;

pub use actions::*;
pub use artist::*;
pub use images::*;
pub use metadata::*;
pub use response::*;
pub use sections::*;
pub use track::*;
