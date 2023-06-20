//! [AsyncCompletionNotifier] and re-export of the tokio [`oneshot`] channel it
//! uses for convenience.

use super::Wrapper;
use crate::NextSample;
use crate::Sound;
pub use tokio::sync::oneshot;

/// Notify via a [tokio::sync::oneshot::Receiver] when the contained Sound has
/// Finished.
pub struct AsyncCompletionNotifier<S: Sound> {
    inner: S,
    sender: Option<oneshot::Sender<()>>,
}

impl<S> AsyncCompletionNotifier<S>
where
    S: Sound,
{
    /// Wrap `inner` so a receiver can be notified when `inner` has `Finished`.
    pub fn new(inner: S) -> (Self, oneshot::Receiver<()>) {
        let (sender, receiver) = oneshot::channel();
        let controllable = AsyncCompletionNotifier {
            inner,
            sender: Some(sender),
        };

        (controllable, receiver)
    }
}

impl<S> Sound for AsyncCompletionNotifier<S>
where
    S: Sound,
{
    fn channel_count(&self) -> u16 {
        self.inner.channel_count()
    }

    fn sample_rate(&self) -> u32 {
        self.inner.sample_rate()
    }

    fn next_sample(&mut self) -> NextSample {
        let next = self.inner.next_sample();
        if let NextSample::Finished = next {
            if let Some(sender) = self.sender.take() {
                // If the consumer dropped their receiver because they don't need it anymore its
                // not an error.
                let _res = sender.send(());
            }
        }
        next
    }

    fn on_start_of_batch(&mut self) {
        self.inner.on_start_of_batch();
    }
}

impl<S> Wrapper for AsyncCompletionNotifier<S>
where
    S: Sound,
{
    type Inner = S;

    fn inner(&self) -> &S {
        &self.inner
    }

    fn inner_mut(&mut self) -> &mut Self::Inner {
        &mut self.inner
    }

    fn into_inner(self) -> S {
        self.inner
    }
}
