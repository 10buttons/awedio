use awedio::Sound;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let Some(file_path) = args() else {
        eprintln!("usage: FILE_PATH");
        std::process::exit(2);
    };

    let (mut manager, _backend) = awedio::start()?;
    let (sound, notifier) = awedio::sounds::open_file(file_path)?.with_completion_notifier();

    manager.play(Box::new(sound));
    let _ = notifier.recv();

    Ok(())
}

fn args() -> Option<String> {
    let mut args = std::env::args();
    args.next()?;
    args.next()
}
