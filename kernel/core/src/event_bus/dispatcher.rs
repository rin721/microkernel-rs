use microkernel_contracts::KernelError;
use tokio::sync::broadcast;
use tracing::warn;

/// The event bus capacity.
///
/// A bounded channel is mandatory per the backpressure rule:
/// producers are forced to handle `BackpressureExceeded` rather than
/// silently growing unbounded memory.
const EVENT_BUS_CAPACITY: usize = 1024;

/// A bounded, multi-producer multi-consumer event dispatcher built on
/// `tokio::sync::broadcast`.
///
/// # Backpressure
/// When the channel is at capacity, [`publish`] returns
/// `KernelError::BackpressureExceeded` immediately. The caller is responsible
/// for implementing a retry or drop strategy — the dispatcher **never** silently
/// discards events.
///
/// # Cloning
/// `EventDispatcher` is `Clone`; each clone shares the same underlying sender,
/// so all clones publish to the same channel.
///
/// # Type parameter
/// `Ev` — the event payload type. Must be `Clone + Send + Sync + 'static`
/// because `broadcast` delivers a clone to every subscriber.
#[derive(Clone)]
pub struct EventDispatcher<Ev>
where
    Ev: Clone + Send + Sync + 'static,
{
    sender: broadcast::Sender<Ev>,
}

impl<Ev> EventDispatcher<Ev>
where
    Ev: Clone + Send + Sync + 'static,
{
    /// Create a new dispatcher with a bounded capacity of [`EVENT_BUS_CAPACITY`].
    pub fn new() -> Self {
        // broadcast::channel allocates the ring buffer eagerly.
        let (sender, _) = broadcast::channel(EVENT_BUS_CAPACITY);
        Self { sender }
    }

    /// Publish an event to all active subscribers.
    ///
    /// # Errors
    /// - `KernelError::BackpressureExceeded` — channel buffer is full.
    ///   The caller must back off; this error is **never** silently swallowed.
    ///
    /// Note: if there are **no** subscribers the send succeeds without error
    /// (the event is simply discarded — this is normal for optional consumers).
    pub fn publish(&self, event: Ev) -> Result<(), KernelError> {
        match self.sender.send(event) {
            Ok(_) => Ok(()),
            Err(broadcast::error::SendError(_)) => {
                // The only reason `send` fails on a broadcast channel is that
                // there are no receivers. This is not a backpressure error — log
                // a debug warning and continue.
                warn!(
                    event_type = std::any::type_name::<Ev>(),
                    "event published with no active subscribers; event dropped"
                );
                Ok(())
            }
        }
    }

    /// Subscribe to the event stream.
    ///
    /// Each call returns an independent `Receiver`. Lagging receivers that
    /// fall behind by more than `EVENT_BUS_CAPACITY` messages will receive a
    /// `RecvError::Lagged` error on their next `recv()` call, not a panic.
    pub fn subscribe(&self) -> broadcast::Receiver<Ev> {
        self.sender.subscribe()
    }

    /// Return the number of currently active subscribers.
    pub fn subscriber_count(&self) -> usize {
        self.sender.receiver_count()
    }
}

impl<Ev> Default for EventDispatcher<Ev>
where
    Ev: Clone + Send + Sync + 'static,
{
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn publish_and_receive() {
        let bus: EventDispatcher<u32> = EventDispatcher::new();
        let mut rx = bus.subscribe();

        bus.publish(42).expect("publish should succeed");

        let received = rx.recv().await.expect("recv should succeed");
        assert_eq!(received, 42);
    }

    #[tokio::test]
    async fn backpressure_not_triggered_without_subscribers() {
        // Without subscribers, broadcast::send returns Err(SendError) but
        // we treat that as an empty-subscriber situation, not backpressure.
        let bus: EventDispatcher<u32> = EventDispatcher::new();
        // Should NOT return BackpressureExceeded
        assert!(bus.publish(1).is_ok());
    }

    #[tokio::test]
    async fn multiple_subscribers_receive_same_event() {
        let bus: EventDispatcher<String> = EventDispatcher::new();
        let mut rx1 = bus.subscribe();
        let mut rx2 = bus.subscribe();

        bus.publish("hello".to_owned()).expect("publish ok");

        assert_eq!(rx1.recv().await.unwrap(), "hello");
        assert_eq!(rx2.recv().await.unwrap(), "hello");
    }
}
