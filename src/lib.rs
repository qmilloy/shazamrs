//! # shazamrs
//!
//! A pure-Rust client for identifying music using Shazam’s public discovery API.
//!
//! This crate is composed of two layers:
//!
//! - **`shazamrs-core`** — audio fingerprinting and signature generation
//! - **`shazamrs`** — network client, response models, and high-level API
//!
//! ## Features
//!
//! - Fully async (`tokio`)
//! - Typed, schema-safe responses
//! - Minimal public API
//!
//! ## Quick start
//!
//! ```no_run
//! use shazamrs::{Shazam, ShazamError};
//!
//! #[tokio::main]
//! async fn main() -> Result<(), ShazamError> {
//!     let shazam = Shazam::new();
//!
//!     let response = shazam.recognize_path("song.mp3").await?;
//!
//!     if let Some(track) = response.track {
//!         println!("🎵 {} — {}",
//!             track.title.unwrap_or_default(),
//!             track.subtitle.unwrap_or_default()
//!         );
//!     }
//!
//!     Ok(())
//! }
//! ```
//!
//! ## Architecture
//!
//! ```text
//! Audio (file/bytes)
//!   ↓
//! shazamrs-core (fingerprinting)
//!   ↓
//! HTTP (Shazam discovery API)
//!   ↓
//! Typed Rust response models
//! ```
//!
//! ## Limitations
//!
//! - This crate relies on Shazam’s **undocumented public API**
//! - Response schemas may change over time
//! - No guarantees of long-term API stability
//!
//! ## Legal notice
//!
//! This project is **not affiliated with or endorsed by Apple or Shazam**.
//! Use at your own risk and comply with applicable laws and terms of service.

#![warn(missing_docs)]

mod client;
mod constants;
mod error;
pub mod models;
mod recognize;

pub use client::Shazam;
pub use error::ShazamError;
