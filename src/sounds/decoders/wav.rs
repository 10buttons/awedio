use std::io::Read;

use crate::sound::NextSample;
use crate::Sound;

use hound::{SampleFormat, WavReader};

// Originally based off Decoder from Rodio.

/// Decoder for the WAV format.
pub struct WavDecoder<R>
where
    R: Read + Send,
{
    reader: WavReader<R>,
    sample_rate: u32,
    channel_count: u16,
}

impl<R> WavDecoder<R>
where
    R: Read + Send,
{
    /// Attempts to decode the data as WAV.
    pub fn new(data: R) -> Result<WavDecoder<R>, hound::Error> {
        let reader = WavReader::new(data)?;
        let spec = reader.spec();

        let sample_rate = spec.sample_rate;
        let channel_count = spec.channels;

        Ok(WavDecoder {
            reader,
            sample_rate,
            channel_count,
        })
    }

    /// Return the wrapped Reader
    pub fn into_inner(self) -> R {
        self.reader.into_inner()
    }
}

impl<R> Sound for WavDecoder<R>
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
        let spec = self.reader.spec();
        let maybe_sample = match (spec.sample_format, spec.bits_per_sample) {
            (SampleFormat::Float, 32) => self
                .reader
                .samples()
                .next()
                .map(|value| value.map(f32_to_i16)),
            (SampleFormat::Int, 8) => self
                .reader
                .samples()
                .next()
                .map(|value| value.map(i8_to_i16)),
            (SampleFormat::Int, 16) => self.reader.samples().next(),
            (SampleFormat::Int, 24) => self
                .reader
                .samples()
                .next()
                .map(|value| value.map(i24_to_i16)),
            (SampleFormat::Int, 32) => self
                .reader
                .samples()
                .next()
                .map(|value| value.map(i32_to_i16)),
            (sample_format, bits_per_sample) => {
                unimplemented!("wav spec: {:?}, {}", sample_format, bits_per_sample)
            }
        };
        match maybe_sample {
            Some(Ok(sample)) => Ok(NextSample::Sample(sample)),
            Some(Err(e)) => Err(e.into()),
            None => Ok(NextSample::Finished),
        }
    }

    fn on_start_of_batch(&mut self) {}
}

// Lossy
fn f32_to_i16(f: f32) -> i16 {
    (f.max(-1.0).min(1.0) * i16::MAX as f32) as i16
}

fn i8_to_i16(i: i8) -> i16 {
    i as i16 * 256
}

// Lossy
fn i24_to_i16(i: i32) -> i16 {
    (i >> 8) as i16
}

// Lossy
fn i32_to_i16(i: i32) -> i16 {
    (i >> 16) as i16
}

impl From<hound::Error> for crate::Error {
    fn from(value: hound::Error) -> Self {
        match value {
            hound::Error::IoError(e) => e.into(),
            hound::Error::FormatError(_)
            | hound::Error::TooWide
            | hound::Error::UnfinishedSample
            | hound::Error::Unsupported
            | hound::Error::InvalidSampleFormat => crate::Error::FormatError(Box::new(value)),
        }
    }
}

#[cfg(test)]
#[path = "./tests/wav.rs"]
mod tests;
