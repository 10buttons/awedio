//! Misc utilities

use std::time::Duration;

/// Convert a number of samples at an old sample rate and channel count to a
/// new number of samples at a different channel rate or sample count such that
/// the new number of samples would be at the same time offset as the old number
/// of samples.
///
/// Any fractional samples are truncated.
///
/// Overflow occurs if old_num_samples * old_channel_count * old_sample_rate
/// does not fit into a u64.
pub fn convert_num_samples(
    old_num_samples: u64,
    old_channel_count: u16,
    old_sample_rate: u32,
    new_channel_count: u16,
    new_sample_rate: u32,
) -> u64 {
    old_num_samples * new_channel_count as u64 * new_sample_rate as u64
        / (old_channel_count as u64 * old_sample_rate as u64)
}

/// Return the number of samples that happen within `duration` amount of time
/// (truncates).
pub fn duration_to_num_samples(duration: Duration, channel_count: u16, sample_rate: u32) -> u64 {
    convert_num_samples(
        duration
            .as_micros()
            .try_into()
            .expect("duration in microseconds is too large to fit into a u64"),
        1,
        1_000_000,
        channel_count,
        sample_rate,
    )
}

#[cfg(test)]
#[path = "./tests/utils.rs"]
mod tests;
