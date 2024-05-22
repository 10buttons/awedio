use super::*;

#[test]
fn test_convert_num_samples() {
    assert_eq!(convert_num_samples(1000, 2, 44100, 2, 44100), 1000);
    assert_eq!(convert_num_samples(1000, 2, 44100, 2, 22050), 500);
    assert_eq!(convert_num_samples(1000, 2, 22050, 2, 44100), 2000);
    assert_eq!(convert_num_samples(1000, 2, 44100, 1, 44100), 500);
    // Truncates fractional samples
    assert_eq!(convert_num_samples(4, 1, 44100, 100, 1), 0);
}

#[test]
fn test_duration_to_num_samples() {
    assert_eq!(
        duration_to_num_samples(Duration::from_millis(1_000), 1, 44100),
        44100
    );
    assert_eq!(
        duration_to_num_samples(Duration::from_millis(1), 1, 44100),
        44
    );
    assert_eq!(
        duration_to_num_samples(Duration::from_millis(10), 1, 44100),
        441
    );
    // 277 hours
    assert_eq!(
        duration_to_num_samples(Duration::from_secs(1_000_000), 6, 44100),
        264_600_000_000
    );
}
