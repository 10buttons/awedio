use crate::sound::NextSample;
use crate::sounds::wrappers::{AddSound, ClearSounds};
use crate::Sound;

/// Play Sounds sequentially one after the other.
///
/// Only after a Sound has returned `NextSample::Finished` will the next Sound
/// start playing.
pub struct SoundList {
    sounds: Vec<Box<dyn Sound>>,
    was_empty: bool,
}

impl SoundList {
    /// Create a new empty SoundList.
    pub fn new() -> Self {
        SoundList {
            sounds: Vec::new(),
            was_empty: false,
        }
    }

    /// Add a Sound to be played after any existing sounds have `Finished`.
    pub fn add(&mut self, sound: Box<dyn Sound>) {
        if self.sounds.is_empty() {
            self.was_empty = true;
        }
        self.sounds.push(sound);
    }

    /// Stop all sounds including the currently playing one.
    pub fn clear(&mut self) {
        self.sounds.clear();
    }
}

impl From<Vec<Box<dyn Sound>>> for SoundList {
    fn from(sounds: Vec<Box<dyn Sound>>) -> Self {
        let was_empty = sounds.is_empty();
        SoundList { sounds, was_empty }
    }
}

impl From<SoundList> for Vec<Box<dyn Sound>> {
    fn from(list: SoundList) -> Self {
        list.sounds
    }
}

// Returned only when no sounds exist so they shouldn't be used in practice.
const DEFAULT_CHANNEL_COUNT: u16 = 2;
const DEFAULT_SAMPLE_RATE: u32 = 44100;

impl Sound for SoundList {
    fn channel_count(&self) -> u16 {
        self.sounds
            .first()
            .map(|s| s.channel_count())
            .unwrap_or(DEFAULT_CHANNEL_COUNT)
    }

    fn sample_rate(&self) -> u32 {
        self.sounds
            .first()
            .map(|s| s.sample_rate())
            .unwrap_or(DEFAULT_SAMPLE_RATE)
    }

    fn on_start_of_batch(&mut self) {
        for sound in &mut self.sounds {
            sound.on_start_of_batch();
        }
    }

    fn next_sample(&mut self) -> NextSample {
        let Some(next_sound) = self.sounds.first_mut() else {
            return NextSample::Finished;
        };
        if self.was_empty {
            self.was_empty = false;
            return NextSample::MetadataChanged;
        }
        let next_sample = next_sound.next_sample();

        match next_sample {
            NextSample::Sample(_) | NextSample::MetadataChanged | NextSample::Paused => next_sample,
            NextSample::Finished => {
                self.sounds.remove(0);
                if self.sounds.is_empty() {
                    NextSample::Finished
                } else {
                    // The next sample might have different metadata. Instead of
                    // normalizing here let downstream normalize.
                    NextSample::MetadataChanged
                }
            }
        }
    }
}

impl AddSound for SoundList {
    fn add(&mut self, sound: Box<dyn Sound>) {
        self.add(sound);
    }
}

impl ClearSounds for SoundList {
    fn clear(&mut self) {
        self.clear();
    }
}

impl Default for SoundList {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
#[path = "./tests/sound_list.rs"]
mod tests;
