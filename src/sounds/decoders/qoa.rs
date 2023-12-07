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

        let QoaItem::FrameHeader(first_frame) = raw_decoder
            .next()
            .ok_or(DecodeError::InvalidFrameHeader)??
        else {
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

    fn next_sample(&mut self) -> Result<NextSample, crate::Error> {
        loop {
            let Some(next_sample) = self.raw_decoder.next() else {
                return Ok(NextSample::Finished);
            };
            let next_sample = next_sample?;

            match next_sample {
                QoaItem::Sample(s) => return Ok(NextSample::Sample(s)),
                QoaItem::FrameHeader(f) => {
                    if f.num_channels as u16 != self.channel_count
                        || f.sample_rate != self.sample_rate
                    {
                        self.channel_count = f.num_channels.into();
                        self.sample_rate = f.sample_rate;
                        return Ok(NextSample::MetadataChanged);
                    }
                    // No metadata change. Continue and read next sample
                    continue;
                }
            }
        }
    }

    fn on_start_of_batch(&mut self) {}
}

impl From<DecodeError> for crate::Error {
    fn from(value: DecodeError) -> Self {
        match value {
            DecodeError::IoError(e) => e.into(),
            DecodeError::NotQoaFile
            | DecodeError::NoSamples
            | DecodeError::InvalidFrameHeader
            | DecodeError::IncompatibleFrame => crate::Error::FormatError(Box::new(value)),
        }
    }
}

#[cfg(test)]
#[path = "./tests/qoa.rs"]
mod tests;
