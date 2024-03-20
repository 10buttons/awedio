use crate::NextSample;
use crate::Sound;
use symphonia::core::audio::{AudioBuffer, AudioBufferRef, Channels, Signal};
use symphonia::core::codecs::{Decoder, DecoderOptions, CODEC_TYPE_NULL};
use symphonia::core::conv::FromSample;
use symphonia::core::errors::Error;
use symphonia::core::formats::{FormatOptions, FormatReader};
use symphonia::core::io::{MediaSource, MediaSourceStream};
use symphonia::core::meta::{Limit, MetadataOptions};
use symphonia::core::probe::Hint;
use symphonia::core::sample::Sample;

/// Decode formats using the Symphonia crate decoders.
pub struct SymphoniaDecoder {
    sample_rate: u32,

    decoder: Box<dyn Decoder>,
    format: Box<dyn FormatReader>,

    channels: Channels,
    track_id: u32,
    next_channel_idx: u16,
    next_sample_idx: usize,
}

impl SymphoniaDecoder {
    /// A decoder for the first track in data that has a recognized codec.
    ///
    /// The track may have multiple channels.
    pub fn new(
        data: Box<dyn MediaSource>,
        extension: Option<&str>,
    ) -> Result<SymphoniaDecoder, Error> {
        let mss = MediaSourceStream::new(data, Default::default());

        let mut hint = Hint::new();
        if let Some(extension) = extension {
            hint.with_extension(extension);
        }
        let meta_opts: MetadataOptions = MetadataOptions {
            limit_metadata_bytes: Limit::Maximum(1),
            limit_visual_bytes: Limit::Maximum(1),
        };
        let fmt_opts: FormatOptions = Default::default();
        let probed = symphonia::default::get_probe().format(&hint, mss, &fmt_opts, &meta_opts)?;

        let format = probed.format;

        // Find the first audio track with a known (decodable) codec.
        let track = format
            .tracks()
            .iter()
            .find(|t| t.codec_params.codec != CODEC_TYPE_NULL)
            .ok_or(Error::Unsupported(
                "No track with a supported codec was found",
            ))?;
        let track_id = track.id;

        let dec_opts: DecoderOptions = Default::default();
        let decoder = symphonia::default::get_codecs().make(&track.codec_params, &dec_opts)?;

        let mut decoder = SymphoniaDecoder {
            sample_rate: 1000,
            decoder,
            format,
            channels: Channels::empty(),
            track_id,
            next_channel_idx: 0,
            next_sample_idx: 0,
        };
        // Ignore metadata changed since no one has seen the old values
        let _ = decoder.decode_next_packet();
        Ok(decoder)
    }
}

impl Sound for SymphoniaDecoder {
    fn channel_count(&self) -> u16 {
        self.channels.count().try_into().unwrap()
    }

    fn sample_rate(&self) -> u32 {
        self.sample_rate
    }

    fn next_sample(&mut self) -> Result<NextSample, crate::Error> {
        if self.next_channel_idx >= self.channels.count().try_into().unwrap() {
            self.next_channel_idx = 0;
            self.next_sample_idx += 1;
        }
        let mut buf_ref = self.decoder.last_decoded();
        if self.next_sample_idx >= buf_ref.frames() {
            match self.decode_next_packet() {
                Ok(true) => return Ok(NextSample::MetadataChanged),
                Ok(false) => (),
                Err(Error::IoError(err))
                    if err.kind() == std::io::ErrorKind::UnexpectedEof
                        && err.to_string() == "end of stream" =>
                {
                    // According to Symphonia this is the only way to detect an end of stream
                    return Ok(NextSample::Finished);
                }
                // TODO: Handle errors better when awedio allows returning errors.
                Err(e) => return Err(e.into()),
            };
            buf_ref = self.decoder.last_decoded();
        }
        let sample = extract_sample_from_ref(&buf_ref, self.next_channel_idx, self.next_sample_idx);
        self.next_channel_idx += 1;
        Ok(NextSample::Sample(sample))
    }

    fn on_start_of_batch(&mut self) {}
}

impl SymphoniaDecoder {
    fn decode_next_packet(&mut self) -> Result<bool, Error> {
        loop {
            let packet = self.format.next_packet()?;
            // We don't currently use the metadata but pop it off so it does not take
            // memory.
            while !self.format.metadata().is_latest() {
                self.format.metadata().pop();
            }
            if packet.track_id() != self.track_id {
                continue;
            }

            // According to the Symphonia, some errors are indeed recoverable:
            let buf_ref = match self.decoder.decode(&packet) {
                Ok(buf_ref) => buf_ref,
                // Recoverable, but this packet is void. Expect weird noises!
                Err(Error::IoError(err)) => {
                    if err.kind() == std::io::ErrorKind::UnexpectedEof
                        && err.to_string() == "end of stream"
                    {
                        // According to Symphonia this is the only way to detect an end of stream
                        return Err(Error::IoError(err));
                    } else {
                        log::warn!("IoError while decoding stream: {}", err);
                        continue;
                    }
                }
                Err(Error::DecodeError(e)) => {
                    log::warn!("DecodeError while decoding stream: {}", e);
                    continue;
                }
                // Reset required, which is handled correctly by this decoder
                Err(Error::ResetRequired) => continue,
                // All other errors are unrecoverable
                Err(e) => return Err(e),
            };

            self.next_channel_idx = 0;
            self.next_sample_idx = 0;
            let mut metadata_changed = false;
            if buf_ref.spec().channels != self.channels {
                self.channels = buf_ref.spec().channels;
                metadata_changed = true;
            }
            if buf_ref.spec().rate != self.sample_rate {
                self.sample_rate = buf_ref.spec().rate;
                metadata_changed = true;
            }
            return Ok(metadata_changed);
        }
    }
}

pub fn extract_sample_from_ref(
    buffer: &AudioBufferRef,
    channel_idx: u16,
    sample_idx: usize,
) -> i16 {
    match buffer {
        AudioBufferRef::U8(buffer) => extract_sample(buffer, channel_idx, sample_idx),
        AudioBufferRef::U16(buffer) => extract_sample(buffer, channel_idx, sample_idx),
        AudioBufferRef::U24(buffer) => extract_sample(buffer, channel_idx, sample_idx),
        AudioBufferRef::U32(buffer) => extract_sample(buffer, channel_idx, sample_idx),
        AudioBufferRef::S8(buffer) => extract_sample(buffer, channel_idx, sample_idx),
        AudioBufferRef::S16(buffer) => extract_sample(buffer, channel_idx, sample_idx),
        AudioBufferRef::S24(buffer) => extract_sample(buffer, channel_idx, sample_idx),
        AudioBufferRef::S32(buffer) => extract_sample(buffer, channel_idx, sample_idx),
        AudioBufferRef::F32(buffer) => extract_sample(buffer, channel_idx, sample_idx),
        AudioBufferRef::F64(buffer) => extract_sample(buffer, channel_idx, sample_idx),
    }
}

pub fn extract_sample<S: Sample>(
    buffer: &AudioBuffer<S>,
    channel_idx: u16,
    sample_idx: usize,
) -> i16
where
    i16: FromSample<S>,
{
    FromSample::from_sample(buffer.chan(channel_idx as usize)[sample_idx])
}

impl From<Error> for crate::Error {
    fn from(value: Error) -> Self {
        match value {
            Error::IoError(e) => e.into(),
            Error::DecodeError(_)
            | Error::SeekError(_)
            | Error::Unsupported(_)
            | Error::LimitError(_)
            | Error::ResetRequired => crate::Error::FormatError(Box::new(value)),
        }
    }
}

#[cfg(test)]
#[path = "./tests/symphonia.rs"]
mod tests;
