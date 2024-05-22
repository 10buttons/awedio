#![warn(missing_docs)]
#![forbid(unsafe_code)]
#![doc = include_str!("../README.md")]

pub mod backends;
pub mod manager;
pub mod sounds;
pub mod utils;

mod error;
mod sound;
#[cfg(test)]
mod tests;

pub use error::Error;
pub use sound::NextSample;
pub use sound::Sound;

/// Start outputting audio with the default backend, device, and configs.
///
/// Currently if an error occurs, it is printed to stderr but not handled in any
/// other way. This should be improved in the future.
/// For more control, create the [CpalBackend][backends::CpalBackend]
/// explicitly.
#[cfg(feature = "cpal")]
pub fn start() -> Result<(manager::Manager, backends::CpalBackend), backends::CpalBackendError> {
    let mut backend =
        backends::CpalBackend::with_defaults().ok_or(backends::CpalBackendError::NoDevice)?;
    let manager = backend.start(|error| eprintln!("error with cpal output stream: {}", error))?;
    Ok((manager, backend))
}
