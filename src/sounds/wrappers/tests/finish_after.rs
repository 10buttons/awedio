use super::*;
use crate::{sounds::wrappers::SetPaused, tests::ConstantValueSound};

#[test]
fn test_simple() {
    let mut sound = ConstantValueSound::new(1000).finish_after(Duration::from_millis(100));
    for _ in 0..(44100 * 2 / 10) {
        assert_eq!(sound.next_sample(), crate::NextSample::Sample(1000));
    }
    assert_eq!(sound.next_sample(), crate::NextSample::Finished);
}

#[test]
fn test_pausing_does_not_count() {
    let mut sound = ConstantValueSound::new(1000)
        .pausable()
        .finish_after(Duration::from_millis(100));
    for s in 0..(44100 * 2 / 10) {
        if s % 15 == 0 {
            sound.set_paused(true);
            assert_eq!(sound.next_sample(), crate::NextSample::Paused);
            sound.set_paused(false);
        }
        assert_eq!(sound.next_sample(), crate::NextSample::Sample(1000));
    }
    assert_eq!(sound.next_sample(), crate::NextSample::Finished);
}

#[test]
fn test_metadata_change_beginning() {
    let mut sound = ConstantValueSound::new(1000).finish_after(Duration::from_millis(100));
    sound.inner_mut().set_sample_rate(22050);
    sound.inner_mut().set_channel_count(1);
    assert_eq!(sound.next_sample(), crate::NextSample::MetadataChanged);
    for _ in 0..(44100 / 2 / 10) {
        assert_eq!(sound.next_sample(), crate::NextSample::Sample(1000));
    }
    assert_eq!(sound.next_sample(), crate::NextSample::Finished);
}

#[test]
fn test_metadata_change_halfway() {
    let mut sound = ConstantValueSound::new(1000).finish_after(Duration::from_millis(100));
    for _ in 0..(44100 / 2 / 5) {
        assert_eq!(sound.next_sample(), crate::NextSample::Sample(1000));
    }
    sound.inner_mut().set_sample_rate(88200);
    sound.inner_mut().set_channel_count(4);
    assert_eq!(sound.next_sample(), crate::NextSample::MetadataChanged);
    for _ in 0..(88200 * 4 / 20) {
        assert_eq!(sound.next_sample(), crate::NextSample::Sample(1000));
    }
    assert_eq!(sound.next_sample(), crate::NextSample::Finished);
}

#[test]
fn test_metadata_change_end() {
    let mut sound = ConstantValueSound::new(1000).finish_after(Duration::from_millis(100));
    for _ in 0..(44100 * 2 / 10) {
        assert_eq!(sound.next_sample(), crate::NextSample::Sample(1000));
    }
    assert_eq!(sound.next_sample(), crate::NextSample::Finished);
    sound.inner_mut().set_sample_rate(22050);
    sound.inner_mut().set_channel_count(1);
    assert_eq!(sound.next_sample(), crate::NextSample::Finished);
}
