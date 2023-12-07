use crate::{
    sounds::wrappers::{SetPaused, SetSpeed},
    tests::Sawtooth,
    NextSample, Sound,
};

use super::*;

#[test]
fn test_no_conversion() {
    let sound = Sawtooth::new(2, 1000).pausable();
    let mut converted = SampleRateConverter::new(sound, 1000);
    assert_eq!(converted.channel_count(), 2);
    assert_eq!(converted.sample_rate(), 1000);
    assert_eq!(converted.next_sample().unwrap(), NextSample::Sample(0));
    assert_eq!(converted.next_sample().unwrap(), NextSample::Sample(0));
    assert_eq!(converted.next_sample().unwrap(), NextSample::Sample(1));
    assert_eq!(converted.next_sample().unwrap(), NextSample::Sample(1));
    assert_eq!(converted.next_sample().unwrap(), NextSample::Sample(2));
    assert_eq!(converted.next_sample().unwrap(), NextSample::Sample(2));
    converted.inner_mut().set_paused(true);
    assert_eq!(converted.next_sample().unwrap(), NextSample::Paused);
    assert_eq!(converted.next_sample().unwrap(), NextSample::Paused);
    converted.inner_mut().set_paused(false);
    assert_eq!(converted.next_sample().unwrap(), NextSample::Sample(3));
    assert_eq!(converted.next_sample().unwrap(), NextSample::Sample(3));
}

#[test]
fn test_four_times() {
    let sound = Sawtooth::new(2, 1000).pausable();
    let mut converted = SampleRateConverter::new(sound, 250);
    assert_eq!(converted.channel_count(), 2);
    assert_eq!(converted.sample_rate(), 250);
    assert_eq!(converted.next_sample().unwrap(), NextSample::Sample(0));
    assert_eq!(converted.next_sample().unwrap(), NextSample::Sample(0));
    assert_eq!(converted.next_sample().unwrap(), NextSample::Sample(4));
    assert_eq!(converted.next_sample().unwrap(), NextSample::Sample(4));
    assert_eq!(converted.next_sample().unwrap(), NextSample::Sample(8));
    converted.inner_mut().set_paused(true);
    // Takes time to flush the buffer. That is okay.
    assert_eq!(converted.next_sample().unwrap(), NextSample::Sample(8));
    assert_eq!(converted.next_sample().unwrap(), NextSample::Paused);
    assert_eq!(converted.next_sample().unwrap(), NextSample::Paused);
    converted.inner_mut().set_paused(false);
    // The pause interrupted our conversion so we come back in
    // a different offset
    assert_eq!(converted.next_sample().unwrap(), NextSample::Sample(10));
    assert_eq!(converted.next_sample().unwrap(), NextSample::Sample(10));
    assert_eq!(converted.next_sample().unwrap(), NextSample::Sample(14));
    assert_eq!(converted.next_sample().unwrap(), NextSample::Sample(14));
}

#[test]
fn test_div_4() {
    let sound = Sawtooth::new(2, 1000).pausable();
    let mut converted = SampleRateConverter::new(sound, 4000);
    assert_eq!(converted.channel_count(), 2);
    assert_eq!(converted.sample_rate(), 4000);
    assert_eq!(converted.next_sample().unwrap(), NextSample::Sample(0));
    assert_eq!(converted.next_sample().unwrap(), NextSample::Sample(0));
    assert_eq!(converted.next_sample().unwrap(), NextSample::Sample(0));
    assert_eq!(converted.next_sample().unwrap(), NextSample::Sample(0));
    assert_eq!(converted.next_sample().unwrap(), NextSample::Sample(0));
    assert_eq!(converted.next_sample().unwrap(), NextSample::Sample(0));
    assert_eq!(converted.next_sample().unwrap(), NextSample::Sample(0));
    assert_eq!(converted.next_sample().unwrap(), NextSample::Sample(0));
    assert_eq!(converted.next_sample().unwrap(), NextSample::Sample(1));
    assert_eq!(converted.next_sample().unwrap(), NextSample::Sample(1));
    assert_eq!(converted.next_sample().unwrap(), NextSample::Sample(1));
    assert_eq!(converted.next_sample().unwrap(), NextSample::Sample(1));
    assert_eq!(converted.next_sample().unwrap(), NextSample::Sample(1));
    assert_eq!(converted.next_sample().unwrap(), NextSample::Sample(1));
    assert_eq!(converted.next_sample().unwrap(), NextSample::Sample(1));
    assert_eq!(converted.next_sample().unwrap(), NextSample::Sample(1));
    assert_eq!(converted.next_sample().unwrap(), NextSample::Sample(2));
    assert_eq!(converted.next_sample().unwrap(), NextSample::Sample(2));
    assert_eq!(converted.next_sample().unwrap(), NextSample::Sample(2));
    assert_eq!(converted.next_sample().unwrap(), NextSample::Sample(2));
    assert_eq!(converted.next_sample().unwrap(), NextSample::Sample(2));
    converted.inner_mut().set_paused(true);
    // It can take some time for us to flush buffers before we
    // see the pause from the inner item. This is fine.
    assert_eq!(converted.next_sample().unwrap(), NextSample::Sample(2));
    assert_eq!(converted.next_sample().unwrap(), NextSample::Sample(2));
    assert_eq!(converted.next_sample().unwrap(), NextSample::Sample(2));
    assert_eq!(converted.next_sample().unwrap(), NextSample::Sample(3));
    assert_eq!(converted.next_sample().unwrap(), NextSample::Sample(3));
    assert_eq!(converted.next_sample().unwrap(), NextSample::Paused);
    assert_eq!(converted.next_sample().unwrap(), NextSample::Paused);
    converted.inner_mut().set_paused(false);
    // Coming back from being paused we can lose samples
    assert_eq!(converted.next_sample().unwrap(), NextSample::Sample(4));
    assert_eq!(converted.next_sample().unwrap(), NextSample::Sample(4));
    assert_eq!(converted.next_sample().unwrap(), NextSample::Sample(4));
    assert_eq!(converted.next_sample().unwrap(), NextSample::Sample(4));
    assert_eq!(converted.next_sample().unwrap(), NextSample::Sample(4));
    assert_eq!(converted.next_sample().unwrap(), NextSample::Sample(4));
    assert_eq!(converted.next_sample().unwrap(), NextSample::Sample(4));
    assert_eq!(converted.next_sample().unwrap(), NextSample::Sample(4));
    assert_eq!(converted.next_sample().unwrap(), NextSample::Sample(5));
}

#[test]
fn catch_metadata_changed_when_passing_through() {
    let sound = Sawtooth::new(1, 1000).with_adjustable_speed();
    let mut converted = SampleRateConverter::new(sound, 1000);
    assert_eq!(converted.channel_count(), 1);
    assert_eq!(converted.sample_rate(), 1000);
    assert_eq!(converted.next_sample().unwrap(), NextSample::Sample(0));
    assert_eq!(converted.next_sample().unwrap(), NextSample::Sample(1));
    converted.inner_mut().set_speed(2.0);
    assert_eq!(
        converted.next_sample().unwrap(),
        NextSample::MetadataChanged
    );
    assert_eq!(converted.sample_rate(), 1000);
    assert_eq!(converted.inner_mut().sample_rate(), 2000);
    assert_eq!(converted.next_sample().unwrap(), NextSample::Sample(2));
    assert_eq!(converted.next_sample().unwrap(), NextSample::Sample(4));
    assert_eq!(converted.next_sample().unwrap(), NextSample::Sample(6));
    assert_eq!(converted.channel_count(), 1);
}
