use crate::tests::Sawtooth;

use super::*;

#[test]
fn test_skip() {
    {
        let mut sound = Sawtooth::new(1, std::u16::MAX as u32);
        sound.skip(Duration::from_millis(500)).unwrap();
        assert_eq!(
            sound.next_sample().unwrap(),
            NextSample::Sample(std::i16::MAX)
        );
    }
    {
        let mut sound = Sawtooth::new(1, std::u16::MAX as u32);
        sound.skip(Duration::from_millis(1000)).unwrap();
        assert_eq!(sound.next_sample().unwrap(), NextSample::Sample(-1));
    }
    // TODO test metadata changed
}
