use super::*;
use crate::NextSample;

const SINE_WAVE_FILE: &[u8] = include_bytes!("audiocheck.net_sin_1000Hz_0dBFS_0.1s.wav");

#[test]
fn samples_of_test_file() -> std::io::Result<()> {
    let mut decoder = WavDecoder::new(std::io::Cursor::new(SINE_WAVE_FILE)).unwrap();
    assert_eq!(decoder.sample_rate(), 44100);
    assert_eq!(decoder.channel_count(), 1);
    assert_eq!(decoder.next_sample(), NextSample::Sample(0)); // 2
    assert_eq!(decoder.next_sample(), NextSample::Sample(4647)); // 2
    assert_eq!(decoder.next_sample(), NextSample::Sample(9201)); // 3
    assert_eq!(decoder.next_sample(), NextSample::Sample(13567)); // 4
    assert_eq!(decoder.next_sample(), NextSample::Sample(17659)); // 5
    assert_eq!(decoder.next_sample(), NextSample::Sample(21393)); // 6
    assert_eq!(decoder.next_sample(), NextSample::Sample(24693)); // 7
    assert_eq!(decoder.next_sample(), NextSample::Sample(27493)); // 8
    assert_eq!(decoder.next_sample(), NextSample::Sample(29736)); // 9
    assert_eq!(decoder.next_sample(), NextSample::Sample(31377)); // 10
    assert_eq!(decoder.next_sample(), NextSample::Sample(32381)); // 11
    assert_eq!(decoder.next_sample(), NextSample::Sample(32729)); // 12
    assert_eq!(decoder.next_sample(), NextSample::Sample(32414)); // 13
    for _i in 0..4398 {
        // println!("i: {_i}");
        let sample = decoder.next_sample();
        match sample {
            NextSample::Sample(_) => {}
            NextSample::MetadataChanged => unreachable!(),
            NextSample::Paused => unreachable!(),
            NextSample::Finished => unreachable!(),
        }
    }
    assert_eq!(decoder.next_sample(), NextSample::Finished);
    assert_eq!(decoder.next_sample(), NextSample::Finished);
    Ok(())
}
