//! Common utilities for tests.
//!
//! Note that other files under the tests/ folders are not submodules of this
//! mod but are submodules of the modules they are testing.

use crate::Sound;

pub const DEFAULT_SAMPLE_RATE: u32 = 44100;
pub const DEFAULT_CHANNEL_COUNT: u16 = 2;

/// Only useful for tests as a constant offset makes no hearable sound.
pub struct ConstantValueSound {
    pub value: i16,
    pub channel_count: u16,
    pub sample_rate: u32,
    pub metadata_changed: bool,
}

impl ConstantValueSound {
    pub fn new(value: i16) -> ConstantValueSound {
        ConstantValueSound {
            value,
            channel_count: DEFAULT_CHANNEL_COUNT,
            sample_rate: DEFAULT_SAMPLE_RATE,
            metadata_changed: false,
        }
    }
}

impl Sound for ConstantValueSound {
    fn channel_count(&self) -> u16 {
        self.channel_count
    }

    fn sample_rate(&self) -> u32 {
        self.sample_rate
    }

    fn next_sample(&mut self) -> crate::NextSample {
        if self.metadata_changed {
            self.metadata_changed = false;
            return crate::NextSample::MetadataChanged;
        }
        crate::NextSample::Sample(self.value)
    }

    fn on_start_of_batch(&mut self) {}
}

impl ConstantValueSound {
    pub fn set_channel_count(&mut self, new_count: u16) {
        self.channel_count = new_count;
        self.metadata_changed = true;
    }

    pub fn set_sample_rate(&mut self, new_rate: u32) {
        self.sample_rate = new_rate;
        self.metadata_changed = true;
    }
}

/// Start at 0, increment by 1 until MAX value then jump to MIN value and
/// increment by 1 again
pub struct Sawtooth {
    pub value: i16,
    pub channel_count: u16,
    pub channel_idx: u16,
    pub sample_rate: u32,
}

impl Sawtooth {
    pub fn new(channel_count: u16, sample_rate: u32) -> Sawtooth {
        Sawtooth {
            value: 0,
            channel_count,
            channel_idx: 0,
            sample_rate,
        }
    }
}

impl Sound for Sawtooth {
    fn channel_count(&self) -> u16 {
        self.channel_count
    }

    fn sample_rate(&self) -> u32 {
        self.sample_rate
    }

    fn next_sample(&mut self) -> crate::NextSample {
        let to_return = crate::NextSample::Sample(self.value);
        self.channel_idx += 1;
        if self.channel_idx == self.channel_count {
            self.channel_idx = 0;
            self.value = self.value.wrapping_add(1);
        }
        to_return
    }

    fn on_start_of_batch(&mut self) {}
}
