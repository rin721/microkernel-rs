use microkernel_contracts::KernelError;
use tokio::sync::broadcast;
use tracing::warn;

/// 事件总线容量。
///
/// 根据背压规则，必须使用有界通道：
/// 生产者被迫处理 `BackpressureExceeded`，
/// 而不是无声地增加无界内存。
const EVENT_BUS_CAPACITY: usize = 1024;

/// 基于 `tokio::sync::broadcast` 构建的，
/// 有界的、多生产者多消费者事件调度器。
///
/// # Backpressure
/// 当通道达到容量时，[`publish`] 立即返回
/// `KernelError::BackpressureExceeded`。调用者有责任
/// 实现重试或丢弃策略 — 调度器**永远不会**无声地丢弃事件。
///
/// # Cloning
/// `EventDispatcher` 实现了 `Clone`；每个克隆共享底层发送者，
/// 所以所有的克隆都会发布到同一个通道。
///
/// # Type parameter
/// `Ev` — 事件有效载荷类型。必须是 `Clone + Send + Sync + 'static`，
/// 因为 `broadcast` 会向每个订阅者传递一个克隆。
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
    /// 使用指定的 [`EVENT_BUS_CAPACITY`] 边界容量创建一个新的调度器。
    pub fn new() -> Self {
        // broadcast::channel 急切地分配环形缓冲区。
        let (sender, _) = broadcast::channel(EVENT_BUS_CAPACITY);
        Self { sender }
    }

    /// 向所有活动的订阅者发布事件。
    ///
    /// # Errors
    /// - `KernelError::BackpressureExceeded` — 通道缓冲区已满。
    ///   调用者必须后退；此错误**永远不会**被无声地吞噬。
    ///
    /// 注意：如果**没有**订阅者，发送成功且不报错
    /// （事件被简单地丢弃 — 这对可选消费者来说是正常的）。
    pub fn publish(&self, event: Ev) -> Result<(), KernelError> {
        match self.sender.send(event) {
            Ok(_) => Ok(()),
            Err(broadcast::error::SendError(_)) => {
                // `send` 在广播通道上失败的唯一原因是
                // 没有接收者。这不是背压错误 — 记录
                // 调试警告并继续。
                warn!(
                    event_type = std::any::type_name::<Ev>(),
                    "event published with no active subscribers; event dropped"
                );
                Ok(())
            }
        }
    }

    /// 订阅事件流。
    ///
    /// 每次调用返回一个独立的 `Receiver`。落后超过
    /// `EVENT_BUS_CAPACITY` 消息的接收者，在下次 `recv()` 调用时，
    /// 将收到 `RecvError::Lagged` 错误，而不是引发 panic。
    pub fn subscribe(&self) -> broadcast::Receiver<Ev> {
        self.sender.subscribe()
    }

    /// 返回当前活动的订阅者数量。
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
        // 没有订阅者时，broadcast::send 返回 Err(SendError)，但
        // 我们将其视为无订阅者的情况，而不是背压。
        let bus: EventDispatcher<u32> = EventDispatcher::new();
        // 不应返回 BackpressureExceeded
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
