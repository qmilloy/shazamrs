//! Response models for the Shazam discovery API.
//!
//! These structs are designed to:
//! - Match Shazam’s JSON responses
//! - Be tolerant of missing or optional fields
//! - Remain forward-compatible as the API evolves
//!
//! All fields are optional unless guaranteed by the backend.

pub mod actions;
pub mod artist;
pub mod images;
pub mod metadata;
pub mod response;
pub mod sections;
pub mod track;

pub use actions::*;
pub use artist::*;
pub use images::*;
pub use metadata::*;
pub use response::*;
pub use sections::*;
pub use track::*;
