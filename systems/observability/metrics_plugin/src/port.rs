use microkernel_contracts::AppError;
use metrics::{counter, gauge, histogram};

pub struct MetricsHandle {}

impl MetricsHandle {
    pub fn inc_counter(&self, name: &str, increment: u64) {
        counter!(name.to_owned()).increment(increment);
    }

    pub fn set_gauge(&self, name: &str, value: f64) {
        gauge!(name.to_owned()).set(value);
    }

    pub fn record_histogram(&self, name: &str, value: f64) {
        histogram!(name.to_owned()).record(value);
    }
}
