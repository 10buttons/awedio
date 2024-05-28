//! [`CpalBackend`] outputs audio using the [cpal](https://www.docs.rs/cpal)
//! crate.

use crate::{manager::BackendSource, manager::Manager, Sound};
use cpal::{
    traits::{DeviceTrait, HostTrait, StreamTrait},
    BuildStreamError, PlayStreamError, StreamError,
};
use std::error::Error;

pub use cpal::BufferSize as CpalBufferSize;

/// A backend that uses [cpal](https://www.docs.rs/cpal) to output to devices.
///
/// This backend does not currently update the output device if the default
/// output device of the host changes.
pub struct CpalBackend {
    channel_count: u16,
    sample_rate: u32,
    buffer_size: CpalBufferSize,
    device: cpal::Device,
    stream: Option<cpal::Stream>,
}

impl CpalBackend {
    /// Create a new CpalBackend with defaults for all fields.
    ///
    /// Returns None if a default device or config could not be obtained.
    pub fn with_defaults() -> Option<CpalBackend> {
        let host = cpal::default_host();

        let device = host.default_output_device()?;

        let default_config = device.default_output_config().ok()?;
        let sample_rate = default_config.sample_rate().0;
        let channel_count = default_config.channels();

        Some(CpalBackend {
            channel_count,
            sample_rate,
            buffer_size: CpalBufferSize::Default,
            device,
            stream: None,
        })
    }

    /// Create a new backend.
    ///
    /// Returns None if an output device is not found
    pub fn with_default_host_and_device(
        channel_count: u16,
        sample_rate: u32,
        buffer_size: CpalBufferSize,
    ) -> Option<CpalBackend> {
        let host = cpal::default_host();

        let device = host.default_output_device()?;

        Some(CpalBackend {
            channel_count,
            sample_rate,
            buffer_size,
            device,
            stream: None,
        })
    }

    /// Create a new CpalBackend specifying all fields.
    pub fn new(
        channel_count: u16,
        sample_rate: u32,
        buffer_size: CpalBufferSize,
        device: cpal::Device,
    ) -> CpalBackend {
        CpalBackend {
            channel_count,
            sample_rate,
            buffer_size,
            device,
            stream: None,
        }
    }
}

impl CpalBackend {
    /// Start a cpal output stream and connect it to the returned Manager.
    ///
    /// Only a single stream is supported at a time per CpalBackend object.
    ///
    /// Cpal stream errors will be reported by calling `error_callback`.
    pub fn start<E>(&mut self, error_callback: E) -> Result<Manager, CpalBackendError>
    where
        E: FnMut(StreamError) + Send + 'static,
    {
        let (manager, mut renderer) = Manager::new();
        renderer.set_output_channel_count_and_sample_rate(self.channel_count, self.sample_rate);
        let Ok(crate::NextSample::MetadataChanged) = renderer.next_sample() else {
            panic!("expected MetadataChanged event")
        };
        let channel_count = self.channel_count;
        let data_callback = move |buffer: &mut [i16], _info: &cpal::OutputCallbackInfo| {
            assert!(buffer.len() % channel_count as usize == 0);
            renderer.on_start_of_batch();

            buffer.fill_with(|| {
                let sample = renderer
                    .next_sample()
                    .expect("renderer should never return an Error");
                match sample {
                    crate::NextSample::Sample(s) => s,
                    crate::NextSample::MetadataChanged => {
                        unreachable!("we never change metadata mid-batch")
                    }
                    // TODO implement pausing
                    crate::NextSample::Paused => 0,
                    // TODO implement finishing
                    crate::NextSample::Finished => 0,
                }
            });
        };

        let config = cpal::StreamConfig {
            channels: self.channel_count,
            sample_rate: cpal::SampleRate(self.sample_rate),
            buffer_size: self.buffer_size.clone(),
        };
        let stream = self
            .device
            .build_output_stream(&config, data_callback, error_callback)?;
        stream.play()?;
        self.stream = Some(stream);
        Ok(manager)
    }
}

/// An error from the [`CpalBackend`]
#[derive(Debug)]
pub enum CpalBackendError {
    /// No output device or configuration found.
    NoDevice,
    /// An error while building the output stream
    BuildStream(BuildStreamError),
    /// An error while starting to play the stream.
    PlayStream(PlayStreamError),
}

impl From<BuildStreamError> for CpalBackendError {
    fn from(inner: BuildStreamError) -> Self {
        CpalBackendError::BuildStream(inner)
    }
}

impl From<PlayStreamError> for CpalBackendError {
    fn from(inner: PlayStreamError) -> Self {
        CpalBackendError::PlayStream(inner)
    }
}

impl std::fmt::Display for CpalBackendError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CpalBackendError::NoDevice => {
                write!(f, "unable to find suitable device or config")
            }
            CpalBackendError::BuildStream(_) => {
                write!(f, "unable to build stream")
            }
            CpalBackendError::PlayStream(_) => {
                write!(f, "unable to play stream")
            }
        }
    }
}

impl Error for CpalBackendError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            CpalBackendError::NoDevice => None,
            CpalBackendError::BuildStream(e) => Some(e),
            CpalBackendError::PlayStream(e) => Some(e),
        }
    }
}
