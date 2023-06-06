use crate::sound::NextSample;
use crate::Sound;
use qoaudio::{DecodeError, QoaDecoder as RawDecoder, QoaItem};
use std::io::Read;

/// Decoder for the [QOA](https://qoaformat.org/) format.
pub struct QoaDecoder<R>
where
    R: Read + Send,
{
    raw_decoder: RawDecoder<R>,
    sample_rate: u32,
    channel_count: u16,
}

impl<R> QoaDecoder<R>
where
    R: Read + Send,
{
    /// Attempts to decode the data as QOA audio.
    pub fn new(data: R) -> Result<QoaDecoder<R>, DecodeError> {
        let mut raw_decoder = RawDecoder::new(data)?;

        let QoaItem::FrameHeader(first_frame) = raw_decoder.next().ok_or(DecodeError::InvalidFrameHeader)?? else {
            return Err(DecodeError::InvalidFrameHeader);
        };
        let sample_rate = first_frame.sample_rate;
        let channel_count = first_frame.num_channels as u16;

        Ok(QoaDecoder {
            raw_decoder,
            sample_rate,
            channel_count,
        })
    }

    /// Return the wrapped Reader
    pub fn into_inner(self) -> R {
        self.raw_decoder.into_inner()
    }
}

impl<R> Sound for QoaDecoder<R>
where
    R: Read + Send,
{
    fn channel_count(&self) -> u16 {
        self.channel_count
    }

    fn sample_rate(&self) -> u32 {
        self.sample_rate
    }

    fn next_sample(&mut self) -> NextSample {
        loop {
            match self.raw_decoder.next() {
                Some(Ok(QoaItem::Sample(s))) => return NextSample::Sample(s),
                Some(Ok(QoaItem::FrameHeader(f))) => {
                    if f.num_channels as u16 != self.channel_count
                        || f.sample_rate != self.sample_rate
                    {
                        self.channel_count = f.num_channels.into();
                        self.sample_rate = f.sample_rate;
                        return NextSample::MetadataChanged;
                    }
                    // No metadata change. Continue and read next sample
                    continue;
                }
                Some(Err(_)) => {
                    // TODO report error somehow
                    return NextSample::Finished;
                }
                None => return NextSample::Finished,
            }
        }
    }

    fn on_start_of_batch(&mut self) {}
}

#[cfg(test)]
#[path = "./tests/qoa.rs"]
mod tests;
