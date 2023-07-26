//! Decoders for various audio formats and file types.
//!
//! These are normally accessed via
//! [sounds::open_file][crate::sounds::open_file()].
#[cfg(feature = "mp3")]
mod mp3;
#[cfg(feature = "qoa")]
mod qoa;
#[cfg(feature = "wav")]
mod wav;

#[cfg(feature = "mp3")]
pub use mp3::Mp3Decoder;
#[cfg(feature = "qoa")]
pub use qoa::QoaDecoder;
#[cfg(feature = "qoa")]
pub use qoaudio::DecodeError as QoaDecodeError;
#[cfg(feature = "wav")]
pub use wav::WavDecoder;
