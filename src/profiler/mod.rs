use serde::{Serialize, Deserialize};

pub mod linux;

#[derive(Serialize, Deserialize)]
pub struct SystemMetrics {
    pub timestamp: chrono::DateTime<chrono::Utc>,
    pub cpu_usage: f32,
    pub memory_usage: u64,
    pub disk_usage: u64,
}

pub trait SystemProfiler {
    fn new() -> Self;
    fn collect_metrics(&self) -> SystemMetrics;
}
