use crate::{NextSample, Sound};

use super::Wrapper;

/// Convert a Sound to have a specified number of output channels.
/// For example convert a mono sound to stereo or vice versa.
pub struct ChannelCountConverter<S: Sound> {
    inner: S,
    to_count: u16,
    converter_type: ConverterType,
}

enum ConverterType {
    PassThrough,
    MonoToStereo { last_sample: Option<i16> },
    StereoToMono,
}

impl<S> ChannelCountConverter<S>
where
    S: Sound,
{
    /// Wrap `inner` such that it will output `to_count` channels.
    pub fn new(inner: S, to_count: u16) -> ChannelCountConverter<S> {
        let converter_type = Self::get_type(inner.channel_count(), to_count);

        ChannelCountConverter {
            inner,
            to_count,
            converter_type,
        }
    }

    fn get_type(from_count: u16, to_count: u16) -> ConverterType {
        if from_count == to_count {
            ConverterType::PassThrough
        } else if from_count == 1 && to_count == 2 {
            ConverterType::MonoToStereo { last_sample: None }
        } else if from_count == 2 && to_count == 1 {
            ConverterType::StereoToMono
        } else {
            // Can implement more conversions like
            // https://developer.mozilla.org/en-US/docs/Web/API/Web_Audio_API/Basic_concepts_behind_Web_Audio_API#up-mixing_and_down-mixing
            todo!(
                "ChannelCountConverter for {} to {} channels not implemented.",
                from_count,
                to_count
            );
        }
    }

    // We could save the metadata of the inner Source and only return MetadataChange
    // if the metadata change is something we can't handle (i.e. a Rate Change).
    fn handle_possible_channel_count_change(&mut self, next: NextSample) {
        if let NextSample::MetadataChanged = next {
            let from_count = self.inner.channel_count();
            self.converter_type = Self::get_type(from_count, self.to_count);
        }
    }

    /// Unwrap the inner Sound.
    ///
    /// It is guaranteed that the inner Sound is at the start of a Frame.
    /// (i.e. the inner sound has not been partially incremented inside a frame)
    pub fn into_inner(self) -> S {
        self.inner
    }
}

impl<S> Sound for ChannelCountConverter<S>
where
    S: Sound,
{
    fn channel_count(&self) -> u16 {
        self.to_count
    }

    fn sample_rate(&self) -> u32 {
        self.inner.sample_rate()
    }

    fn next_sample(&mut self) -> Result<NextSample, crate::Error> {
        match &mut self.converter_type {
            ConverterType::PassThrough => {
                let next = self.inner.next_sample()?;
                self.handle_possible_channel_count_change(next);
                Ok(next)
            }
            ConverterType::MonoToStereo {
                ref mut last_sample,
            } => {
                if let Some(sample) = last_sample {
                    let sample = *sample;
                    *last_sample = None;
                    Ok(NextSample::Sample(sample))
                } else {
                    let next = self.inner.next_sample()?;
                    match next {
                        NextSample::Sample(sample) => {
                            *last_sample = Some(sample);
                        }
                        NextSample::MetadataChanged => {} // handled below
                        NextSample::Paused | NextSample::Finished => {} // Just pass through
                    }
                    self.handle_possible_channel_count_change(next);
                    Ok(next)
                }
            }
            ConverterType::StereoToMono => {
                let next1 = self.inner.next_sample()?;
                self.handle_possible_channel_count_change(next1);
                let sample1 = match next1 {
                    NextSample::Sample(s) => s,
                    NextSample::MetadataChanged | NextSample::Paused | NextSample::Finished => {
                        return Ok(next1);
                    }
                };
                let next2 = self.inner.next_sample()?;
                self.handle_possible_channel_count_change(next2);
                let sample2 = match next2 {
                    NextSample::Sample(s) => s,
                    NextSample::MetadataChanged | NextSample::Paused | NextSample::Finished => {
                        return Ok(next2);
                    }
                };

                // Get the average of the two
                let avg = ((sample1 as i32 + sample2 as i32) / 2) as i16;
                Ok(NextSample::Sample(avg))
            }
        }
    }

    fn on_start_of_batch(&mut self) {
        self.inner.on_start_of_batch()
    }
}

impl<S: Sound> Wrapper for ChannelCountConverter<S> {
    type Inner = S;

    fn inner(&self) -> &S {
        &self.inner
    }

    fn inner_mut(&mut self) -> &mut Self::Inner {
        &mut self.inner
    }

    fn into_inner(self) -> S {
        self.inner
    }
}
