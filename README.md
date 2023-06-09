# Awedio &emsp; [![Docs Passing]][docs.rs] [![Latest Version]][crates.io]

A low-overhead and adaptable audio playback library for Rust.

## Examples

Play a single sound file:

```rust no_run
let (mut manager, backend) = awedio::start()?;
manager.play(awedio::sounds::open_file("test.wav")?);
# Ok::<(), Box<dyn std::error::Error>>(())
```

Play a sound with adjustable volume controllable after playback has started:

```rust no_run
use awedio::Sound;
let (mut manager, backend) = awedio::start()?;
let (sound, mut controller) = awedio::sounds::SineWav::new(400.0)
    .with_adjustable_volume_of(0.25)
    .pausable()
    .controllable();
manager.play(Box::new(sound));
std::thread::sleep(std::time::Duration::from_millis(100));
controller.set_volume(0.5);
std::thread::sleep(std::time::Duration::from_millis(100));
controller.set_paused(true);
# Ok::<(), Box<dyn std::error::Error>>(())
```

## Design Goals

- Usable in low resource environments such as the esp32 microcontroller.
  Currently does require std.
- Very low overhead. For example, a Wav file with i16 samples with the same
  same sample rate as the output device will have samples sent to the backend
  unchanged.
- Only pay the performance cost of features if they are needed. For example
  pausability, volume adjustment, or controlling a sound after playback has
  started are all added to a Sound only as needed. This is done by wrapping
  types that implement [`Sound`] similar to `Iterator` in the standard library.
- Samples are i16 for simplicity at least for now (if this prevents you from
  using this library please let me know your use case)

## API Overview

- [Sound] trait - Provides samples of a sound to be played. Has
  functions to modify sounds with wrappers.
- [Manager] - Play Sounds to a backend.
- [SoundList] - A sequence of Sounds to play one after the other.

## Current backends

- [cpal] - For popular environments such as Linux,
  Windows, Mac OS, Android...  Enabled by the `cpal` feature (on by default).
- [esp32][awedio_esp32] - For esp32 microcontrollers using
  esp-idf. Implemented in its [own crate][awedio_esp32].

Backends are implemented by pulling samples from the [Renderer].

## Cargo Features

- `async`: Enable async features that depend on
  [tokio-sync](https://docs.rs/tokio/latest/tokio/sync/index.html)
- `cpal`: Enable the [cpal] backend.
- `wav`: Enable Wav file decoding using [Hound](https://crates.io/crates/hound)
- `mp3`: Enable mp3 file decoding using [rmp3](https://crates.io/crates/rmp3)

By default all features are enabled. Depending libraries should disable default features.

## Motivation

Built for creating activities for [10 Buttons](https://www.10Buttons.com), a
screen-less tablet for kids. Purposefully kept generic to be usable in other
contexts.

## Alternatives and Inspiration

Thanks to the following audio playback libraries which inspired and were
a reference for this library:

- [Rodio](https://docs.rs/rodio/)
  - Very popular crate for audio playback with Rust.
  - Tightly coupled to cpal.
  - Has a [Sink](https://docs.rs/rodio/latest/rodio/#sink) which is similar to
    `SoundList::new().pausable().with_adjustable_volume().controllable()`...
  - mp3 libraries did not work for me on embedded.
  - Default mixer converts everything to an f32.
  - Uses the standard Iterator trait for its Source/Sound.
- [Kira](https://docs.rs/kira/)
  - Has very nice API for tweening and effects.
  - [Sound](https://docs.rs/kira/latest/kira/sound/trait.Sound.html) trait
    requires a left/right track instead of supporting 1 or N tracks.
  - Samples are all f32.
  - Uses symphonia for audio format parsing and has several internal buffers
    requiring more memory.
  - All samples are resampled based on the timestamp and sample rate.

## License

This project is licensed under either of
[Apache License, Version 2.0](https://www.apache.org/licenses/LICENSE-2.0) or
[MIT license](https://opensource.org/licenses/MIT) at your option.

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall
be dual licensed as above, without any additional terms or conditions.

[Latest Version]: https://img.shields.io/crates/v/awedio.svg
[crates.io]: https://crates.io/crates/awedio
[Docs Passing]: https://img.shields.io/docsrs/awedio.svg
[docs.rs]: https://docs.rs/awedio
[cpal]: https://crates.io/crates/cpal
[awedio_esp32]: https://crates.io/crates/awedio_esp32
[Sound]: https://docs.rs/awedio/latest/awedio/trait.Sound.html
[Manager]: https://docs.rs/awedio/latest/awedio/manager/struct.Manager.html
[SoundList]: https://docs.rs/awedio/latest/awedio/sounds/struct.SoundList.html
[Renderer]: https://docs.rs/awedio/latest/awedio/manager/struct.Renderer.html