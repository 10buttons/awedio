//! Manager is how sounds are played on a backend.
mod renderer;

pub use crate::manager::renderer::Renderer;
use crate::sounds::wrappers::Controllable;
use crate::sounds::wrappers::Controller;
use crate::sounds::SoundMixer;
use crate::Sound;

/// A Manager can play sounds by rendering sounds on a [`Renderer`] for a
/// backend.
#[derive(Clone)]
pub struct Manager {
    mixer_controller: Controller<SoundMixer>,
}

// These are undocumented, should not be relied on and subject to change.
// Backend implementations should set their values at startup.
const DEFAULT_CHANNEL_COUNT: u16 = 1;
const DEFAULT_SAMPLE_RATE: u32 = 1000; // Purposely low value to discourage use

impl Manager {
    /// Create a new Manager and the renderer its samples will render to.
    ///
    /// Normally you do not need to call this function directly but you instead
    /// call `.start(...)` on a backend which will call this function.
    pub fn new() -> (Self, Renderer) {
        let (mixer, mixer_controller) =
            Controllable::new(SoundMixer::new(DEFAULT_CHANNEL_COUNT, DEFAULT_SAMPLE_RATE));
        let renderer = Renderer::new(mixer);
        let manager = Manager { mixer_controller };
        (manager, renderer)
    }

    /// Add a new Sound to be played in parallel to any existing sounds.
    ///
    /// If you want to play Sounds sequentially use a
    /// [SoundList][crate::sounds::SoundList].
    ///
    /// See the modifier functions on [Sound] to control sounds before and/or
    /// after playing.
    pub fn play(&mut self, sound: Box<dyn Sound>) {
        self.mixer_controller.add(sound);
    }

    /// Stop playing and remove all audio sounds. New sounds can still be added.
    pub fn clear(&mut self) {
        self.mixer_controller.clear();
    }
}

impl std::fmt::Debug for Manager {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Manager").finish()
    }
}
