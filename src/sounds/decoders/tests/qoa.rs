use super::*;
use crate::NextSample;

const SINE_WAVE_FILE: &[u8] = include_bytes!("audiocheck.net_sin_1000Hz_0dBFS_0.1s.qoa");

#[test]
fn samples_of_test_file() -> std::io::Result<()> {
    let mut decoder = QoaDecoder::new(std::io::Cursor::new(SINE_WAVE_FILE)).unwrap();
    assert_eq!(decoder.sample_rate(), 44100);
    assert_eq!(decoder.channel_count(), 1);
    assert_eq!(decoder.next_sample().unwrap(), NextSample::Sample(422)); // 1
    assert_eq!(decoder.next_sample().unwrap(), NextSample::Sample(4779)); // 2
    assert_eq!(decoder.next_sample().unwrap(), NextSample::Sample(8886)); // 3
    assert_eq!(decoder.next_sample().unwrap(), NextSample::Sample(13834)); // 4
    assert_eq!(decoder.next_sample().unwrap(), NextSample::Sample(17173)); // 5
    assert_eq!(decoder.next_sample().unwrap(), NextSample::Sample(21539)); // 6
    assert_eq!(decoder.next_sample().unwrap(), NextSample::Sample(24403)); // 7
    assert_eq!(decoder.next_sample().unwrap(), NextSample::Sample(27482)); // 8
    assert_eq!(decoder.next_sample().unwrap(), NextSample::Sample(29200)); // 9
    assert_eq!(decoder.next_sample().unwrap(), NextSample::Sample(31270)); // 10
    assert_eq!(decoder.next_sample().unwrap(), NextSample::Sample(31976)); // 11
    assert_eq!(decoder.next_sample().unwrap(), NextSample::Sample(32767)); // 12
    assert_eq!(decoder.next_sample().unwrap(), NextSample::Sample(32183)); // 13
    for _i in 0..4398 {
        let sample = decoder.next_sample().unwrap();
        match sample {
            NextSample::Sample(_) => {}
            NextSample::MetadataChanged => unreachable!(),
            NextSample::Paused => unreachable!(),
            NextSample::Finished => unreachable!(),
        }
    }
    assert_eq!(decoder.next_sample().unwrap(), NextSample::Finished);
    assert_eq!(decoder.next_sample().unwrap(), NextSample::Finished);
    Ok(())
}
