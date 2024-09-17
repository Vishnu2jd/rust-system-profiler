use crate::profiler::SystemMetrics;

pub mod file;

pub trait Storage {
    fn new(path: &str) -> Self;
    fn store(&self, metrics: &SystemMetrics);
    fn load_all(&self) -> Vec<SystemMetrics>;
}
