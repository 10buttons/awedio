# Changelog

## 0.3.0 - 2023-12-07

- docs: update README
- feat!: change Sound::next_sample to return Result
- feat!: add Symphonia decoder which supports many formats and codecs
  Note that feature flags were renamed to remove ambiguity.
- feat: add finish after sound wrapper
- docs: fix ambiguous doc reference
- docs: add basic play example in examples/
- feat: add CompletionNotifier (blocking, non-async)
- fix: clear paused sounds too in SoundMixer
- feat: re-export tokio oneshot in async_completion_notifier mod
- feat: add Sound::paused which starts paused
- feat: add QOA audio format decoder
- feat: make Controller::send_command public

## 0.2.0 - 2023-05-11

- update README links
- rename UnexpectedMetadataChange to UnsupportedMetadataChangeError

## 0.1.1 - 2023-05-10

- Improved documentation

## 0.1.0 - 2023-05-09

- Initial release
