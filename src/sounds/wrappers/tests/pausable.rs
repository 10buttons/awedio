use super::*;
use crate::tests::ConstantValueSound;

#[test]
fn set_paused_and_unpause() {
    let mut first = ConstantValueSound::new(1000).pausable();
    // starts unpaused
    assert_eq!(first.next_sample(), crate::NextSample::Sample(1000));
    first.set_paused(true);
    assert_eq!(first.next_sample(), crate::NextSample::Paused);
    first.set_paused(false);
    assert_eq!(first.next_sample(), crate::NextSample::Sample(1000));
}
