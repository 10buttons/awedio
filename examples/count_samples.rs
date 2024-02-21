use awedio::{NextSample, Sound};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let Some(file_path) = args() else {
        eprintln!("usage: FILE_PATH");
        std::process::exit(2);
    };

    let mut sound = awedio::sounds::open_file(file_path)?;

    let mut num_samples = 0;

    loop {
        match sound.next_sample() {
            Ok(NextSample::Sample(_)) => num_samples += 1,
            Ok(NextSample::Paused) => {
                println!("Encountered a Pause. Stopping.");
                break;
            }
            Ok(NextSample::Finished) => {
                break;
            }
            Ok(NextSample::MetadataChanged) => {
                println!(
                    "Encountered MetadataChanged. New sample rate: {}, New channel count: {}",
                    sound.sample_rate(),
                    sound.channel_count()
                );
            }
            Err(e) => {
                println!("Encountered error: {:?}", e);
                break;
            }
        }
    }
    println!("Read {} samples.", num_samples);

    Ok(())
}

fn args() -> Option<String> {
    let mut args = std::env::args();
    args.next()?;
    args.next()
}
