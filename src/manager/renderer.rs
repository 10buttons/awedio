use crate::sounds::wrappers::Controllable;
use crate::sounds::wrappers::Wrapper;
use crate::sounds::SoundMixer;
use crate::NextSample;
use crate::Sound;

/// Final mixed samples are pulled from the Renderer.
///
/// Samples are delivered via the Renderer to a backend.
/// Different backends are implemented in external crates for different
/// platforms and use cases.
///
/// The backend is responsible for:
///
///  1. storing the renderer
///  2. calling renderer.set_output_channel_count_and_sample_rate()
///  3. periodically calling renderer.on_start_of_batch() followed by some
///     number of renderer.next_sample() calls (normally enough to fill
///     some number of milliseconds of an output buffer).
pub struct Renderer {
    mixer: Controllable<SoundMixer>,
}

impl Renderer {
    pub(crate) fn new(mixer: Controllable<SoundMixer>) -> Self {
        Renderer { mixer }
    }

    /// Set the output channel count and sample rate that the rendered should
    /// return to the backend.
    pub fn set_output_channel_count_and_sample_rate(
        &mut self,
        output_channel_count: u16,
        output_sample_rate: u32,
    ) {
        self.mixer
            .inner_mut()
            .set_output_channel_count_and_sample_rate(output_channel_count, output_sample_rate);
    }
}

impl Sound for Renderer {
    fn channel_count(&self) -> u16 {
        self.mixer.channel_count()
    }

    fn sample_rate(&self) -> u32 {
        self.mixer.sample_rate()
    }

    /// Get the next sample.
    /// `MetadataChanged` will only be returned from Renderer if
    /// `set_output_channel_count_and_sample_rate` was called. If `Paused` is
    /// returned the backend may choose to pause itself or play silence (e.g.
    /// `.next_sample().unwrap_or(0)`). `Finished` will be returned if no sounds
    /// are playing and the Manager of the Renderer has been dropped.
    fn next_sample(&mut self) -> NextSample {
        self.mixer.next_sample()
    }

    /// Inform the playing or queued sounds that a new batch of samples will be
    /// requested. This must only be called when the next sample to be delivered
    /// from `next_sample` is for the first channel.
    ///
    /// See [Sound::on_start_of_batch]
    fn on_start_of_batch(&mut self) {
        self.mixer.on_start_of_batch()
    }
}
