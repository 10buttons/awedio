use crate::{sounds::SoundList, NextSample, Sound};

use super::*;

#[test]
fn metadata_change_two_off_does_not_cause_desync() {
    let first = MemorySound::from_samples(Arc::new(vec![1, 2, 3, 4, 1, 2]), 4, 1000);
    let second = MemorySound::from_samples(Arc::new(vec![1, 2, 3, 4]), 4, 1000);
    let mut list = SoundList::new();
    list.add(Box::new(first));
    list.add(Box::new(second));
    let mut combined = list.into_memory_sound().unwrap();
    assert_eq!(combined.sample_rate(), 1000);
    assert_eq!(combined.channel_count(), 4);
    assert_eq!(combined.next_sample(), NextSample::Sample(1));
    assert_eq!(combined.next_sample(), NextSample::Sample(2));
    assert_eq!(combined.next_sample(), NextSample::Sample(3));
    assert_eq!(combined.next_sample(), NextSample::Sample(4));
    assert_eq!(combined.next_sample(), NextSample::Sample(1));
    assert_eq!(combined.next_sample(), NextSample::Sample(2));
    assert_eq!(combined.next_sample(), NextSample::Sample(0));
    assert_eq!(combined.next_sample(), NextSample::Sample(0));
    assert_eq!(combined.next_sample(), NextSample::Sample(1));
    assert_eq!(combined.next_sample(), NextSample::Sample(2));
    assert_eq!(combined.next_sample(), NextSample::Sample(3));
    assert_eq!(combined.next_sample(), NextSample::Sample(4));
    assert_eq!(combined.next_sample(), NextSample::Finished);
}

#[test]
fn metadata_change_one_off_does_not_cause_desync() {
    let first = MemorySound::from_samples(Arc::new(vec![1, 2, 3, 4, 1]), 4, 1000);
    let second = MemorySound::from_samples(Arc::new(vec![1, 2, 3, 4]), 4, 1000);
    let mut list = SoundList::new();
    list.add(Box::new(first));
    list.add(Box::new(second));
    let mut combined = MemorySound::from_sound(list).unwrap();
    assert_eq!(combined.sample_rate(), 1000);
    assert_eq!(combined.channel_count(), 4);
    assert_eq!(combined.next_sample(), NextSample::Sample(1));
    assert_eq!(combined.next_sample(), NextSample::Sample(2));
    assert_eq!(combined.next_sample(), NextSample::Sample(3));
    assert_eq!(combined.next_sample(), NextSample::Sample(4));
    assert_eq!(combined.next_sample(), NextSample::Sample(1));
    assert_eq!(combined.next_sample(), NextSample::Sample(0));
    assert_eq!(combined.next_sample(), NextSample::Sample(0));
    assert_eq!(combined.next_sample(), NextSample::Sample(0));
    assert_eq!(combined.next_sample(), NextSample::Sample(1));
    assert_eq!(combined.next_sample(), NextSample::Sample(2));
    assert_eq!(combined.next_sample(), NextSample::Sample(3));
    assert_eq!(combined.next_sample(), NextSample::Sample(4));
    assert_eq!(combined.next_sample(), NextSample::Finished);
}

#[test]
fn metadata_change_in_sync() {
    let first = MemorySound::from_samples(Arc::new(vec![1, 2, 3, 4]), 4, 1000);
    let second = MemorySound::from_samples(Arc::new(vec![1, 2, 3, 4]), 4, 1000);
    let mut list = SoundList::new();
    list.add(Box::new(first));
    list.add(Box::new(second));
    let mut combined = MemorySound::from_sound(list).unwrap();
    assert_eq!(combined.sample_rate(), 1000);
    assert_eq!(combined.channel_count(), 4);
    assert_eq!(combined.next_sample(), NextSample::Sample(1));
    assert_eq!(combined.next_sample(), NextSample::Sample(2));
    assert_eq!(combined.next_sample(), NextSample::Sample(3));
    assert_eq!(combined.next_sample(), NextSample::Sample(4));
    assert_eq!(combined.next_sample(), NextSample::Sample(1));
    assert_eq!(combined.next_sample(), NextSample::Sample(2));
    assert_eq!(combined.next_sample(), NextSample::Sample(3));
    assert_eq!(combined.next_sample(), NextSample::Sample(4));
    assert_eq!(combined.next_sample(), NextSample::Finished);
}

#[test]
fn loop_forever() {
    let mut sound = MemorySound::from_samples(Arc::new(vec![1, 2]), 2, 1000);
    sound.set_looping(true);
    assert_eq!(sound.next_sample(), NextSample::Sample(1));
    assert_eq!(sound.next_sample(), NextSample::Sample(2));
    assert_eq!(sound.next_sample(), NextSample::Sample(1));
    assert_eq!(sound.next_sample(), NextSample::Sample(2));
    assert_eq!(sound.next_sample(), NextSample::Sample(1));
}
