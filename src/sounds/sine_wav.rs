use std::f32::consts::PI;

/// A constant pitch sound of infinite length.
pub struct SineWav {
    freq: f32,
    sample_rate: u32,
    sample_num: u32,
    reset_num: u32,
}

impl SineWav {
    /// A constant pitch sound with a default sample rate of 44,100.
    pub fn new(freq: f32) -> SineWav {
        Self::with_sample_rate(freq, 44100)
    }

    /// A constant pitch sound with `sample_rate`.
    pub fn with_sample_rate(freq: f32, sample_rate: u32) -> SineWav {
        let reset_num = sample_rate as f32 / freq;
        // Find the largest multiple of reset_num that fits into sample_num range of u32
        // so the artifact of resetting happens as little as possible (about once per 27
        // hours)
        let cycles = (u32::MAX as f32 / reset_num).floor();
        let reset_num = (reset_num * cycles).round() as u32;

        SineWav {
            freq,
            sample_rate,
            sample_num: 0,
            reset_num,
        }
    }
}

impl crate::Sound for SineWav {
    fn channel_count(&self) -> u16 {
        1
    }

    fn sample_rate(&self) -> u32 {
        self.sample_rate
    }

    fn next_sample(&mut self) -> crate::NextSample {
        let value = 2.0 * self.sample_num as f32 * self.freq * PI / self.sample_rate as f32;
        if self.sample_num == self.reset_num {
            self.sample_num = 0;
        } else {
            self.sample_num += 1;
        }
        crate::NextSample::Sample((value.sin() * i16::MAX as f32) as i16)
    }

    fn on_start_of_batch(&mut self) {}
}

#[cfg(test)]
#[path = "./tests/sine_wav.rs"]
mod tests;
