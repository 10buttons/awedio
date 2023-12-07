use super::*;
use crate::tests::ConstantValueSound;

#[test]
fn adjust_down() {
    let mut first = ConstantValueSound::new(1000).with_adjustable_volume();
    first.set_volume(0.5);
    assert_eq!(first.next_sample().unwrap(), crate::NextSample::Sample(500));
}

#[test]
fn adjust_up() {
    let mut first = ConstantValueSound::new(1000).with_adjustable_volume();
    first.set_volume(5.0);
    assert_eq!(
        first.next_sample().unwrap(),
        crate::NextSample::Sample(5000)
    );
}

#[test]
fn test_saturation() {
    let mut first = ConstantValueSound::new(1000).with_adjustable_volume();
    first.set_volume(1000.0);
    assert_eq!(
        first.next_sample().unwrap(),
        crate::NextSample::Sample(i16::MAX)
    );
}
