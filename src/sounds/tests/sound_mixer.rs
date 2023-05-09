use super::*;
use crate::tests::{ConstantValueSound, DEFAULT_CHANNEL_COUNT, DEFAULT_SAMPLE_RATE};

#[test]
fn additional_silent_sounds_do_not_affect_first() {
    let first = ConstantValueSound::new(5);
    let second = ConstantValueSound::new(0);
    let mut mixer = SoundMixer::new(DEFAULT_CHANNEL_COUNT, DEFAULT_SAMPLE_RATE);
    mixer.add(Box::new(first));
    mixer.add(Box::new(second));
    assert_eq!(mixer.next_sample(), NextSample::Sample(5));
    assert_eq!(mixer.next_sample(), NextSample::Sample(5));
    assert_eq!(mixer.next_sample(), NextSample::Sample(5));
    let third = ConstantValueSound::new(0);
    mixer.add(Box::new(third));
    assert_eq!(mixer.next_sample(), NextSample::Sample(5));
    assert_eq!(mixer.next_sample(), NextSample::Sample(5));
    assert_eq!(mixer.next_sample(), NextSample::Sample(5));
}

#[test]
fn two_sounds_add_together() {
    let first = ConstantValueSound::new(5);
    let second = ConstantValueSound::new(7);
    let mut mixer = SoundMixer::new(DEFAULT_CHANNEL_COUNT, DEFAULT_SAMPLE_RATE);
    mixer.add(Box::new(first));
    mixer.add(Box::new(second));
    assert_eq!(mixer.next_sample(), NextSample::Sample(12));
    assert_eq!(mixer.next_sample(), NextSample::Sample(12));
    assert_eq!(mixer.next_sample(), NextSample::Sample(12));
}
