use super::*;
use crate::NextSample;

const SINE_WAVE_FILE: &[u8] = include_bytes!("audiocheck.net_sin_1000Hz_0dBFS_0.1s.mp3");

#[test]
fn samples_of_test_file() -> std::io::Result<()> {
    let mut decoder =
        SymphoniaDecoder::new(Box::new(std::io::Cursor::new(SINE_WAVE_FILE)), None).unwrap();
    assert_eq!(decoder.sample_rate(), 44100);
    assert_eq!(decoder.channel_count(), 1);
    for _i in 0..1106 {
        println!("i: {_i}");
        let sample = decoder.next_sample();
        match sample {
            NextSample::Sample(s) => {
                println!("s: {s}");
                assert!(s.abs() < 700)
            }
            NextSample::MetadataChanged => unreachable!(),
            NextSample::Paused => unreachable!(),
            NextSample::Finished => unreachable!(),
        }
    }
    assert_eq!(decoder.next_sample(), NextSample::Sample(4235)); // 2
    assert_eq!(decoder.next_sample(), NextSample::Sample(8784)); // 3
    assert_eq!(decoder.next_sample(), NextSample::Sample(12773)); // 4
    assert_eq!(decoder.next_sample(), NextSample::Sample(16552)); // 5
    assert_eq!(decoder.next_sample(), NextSample::Sample(20398)); // 6
    assert_eq!(decoder.next_sample(), NextSample::Sample(23584)); // 7
    assert_eq!(decoder.next_sample(), NextSample::Sample(25960)); // 8
    assert_eq!(decoder.next_sample(), NextSample::Sample(28079)); // 9
    assert_eq!(decoder.next_sample(), NextSample::Sample(29853)); // 10
    assert_eq!(decoder.next_sample(), NextSample::Sample(30799)); // 11
    assert_eq!(decoder.next_sample(), NextSample::Sample(31009)); // 12
    assert_eq!(decoder.next_sample(), NextSample::Sample(30770)); // 13
    for _i in 0..4642 {
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
