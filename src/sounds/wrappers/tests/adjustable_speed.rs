use super::*;
use crate::tests::{ConstantValueSound, DEFAULT_SAMPLE_RATE};

#[test]
fn adjust_down() {
    let mut first = ConstantValueSound::new(1000).with_adjustable_speed_of(0.5);
    assert_eq!(
        first.next_sample().unwrap(),
        crate::NextSample::Sample(1000)
    );
    assert_eq!(first.sample_rate(), 22050)
}

#[test]
fn adjust_up() {
    let mut first = ConstantValueSound::new(1000).with_adjustable_speed_of(5.0);
    assert_eq!(
        first.next_sample().unwrap(),
        crate::NextSample::Sample(1000)
    );
    assert_eq!(first.sample_rate(), 44100 * 5)
}

#[test]
fn test_real_fast() {
    let mut first = ConstantValueSound::new(1000).with_adjustable_speed_of(1000.0);
    assert_eq!(
        first.next_sample().unwrap(),
        crate::NextSample::Sample(1000)
    );
    assert_eq!(first.sample_rate(), 44100 * 1000)
}

#[test]
fn test_max_saturation() {
    let mut first = ConstantValueSound::new(1000).with_adjustable_speed_of(1_000_000.0);
    assert_eq!(
        first.next_sample().unwrap(),
        crate::NextSample::Sample(1000)
    );
    assert_eq!(first.sample_rate(), u32::MAX)
}

#[test]
fn test_min_saturation() {
    let mut first = ConstantValueSound::new(1000).with_adjustable_speed();
    first.set_speed(0.0000000001);
    assert_eq!(first.sample_rate(), 1);
    assert_eq!(
        first.next_sample().unwrap(),
        crate::NextSample::MetadataChanged
    );
    assert_eq!(
        first.next_sample().unwrap(),
        crate::NextSample::Sample(1000)
    );
}

#[test]
fn metadata_changed_notification() {
    let mut first = ConstantValueSound::new(1000).with_adjustable_speed();
    assert_eq!(first.sample_rate(), DEFAULT_SAMPLE_RATE);
    assert_eq!(
        first.next_sample().unwrap(),
        crate::NextSample::Sample(1000)
    );
    first.set_speed(0.50);
    assert_eq!(first.sample_rate(), 22050);
    assert_eq!(
        first.next_sample().unwrap(),
        crate::NextSample::MetadataChanged
    );
    assert_eq!(
        first.next_sample().unwrap(),
        crate::NextSample::Sample(1000)
    );
}
