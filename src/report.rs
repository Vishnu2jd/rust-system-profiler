use crate::profiler::SystemMetrics;
use crate::storage::Storage;

pub fn print_snapshot(metrics: &SystemMetrics) {
    println!("Timestamp: {}", metrics.timestamp);
    println!("CPU Usage: {:.2}%", metrics.cpu_usage);
    println!("Memory Usage: {} bytes", metrics.memory_usage);
    println!("Disk Usage: {} bytes", metrics.disk_usage);
    println!("--------------------");
}

pub fn generate_summary<T: Storage>(storage: &T) {
    let metrics = storage.load_all();

    let avg_cpu = metrics.iter().map(|m| m.cpu_usage).sum::<f32>() / metrics.len() as f32;
    let avg_memory = metrics.iter().map(|m| m.memory_usage).sum::<u64>() / metrics.len() as u64;
    let avg_disk = metrics.iter().map(|m| m.disk_usage).sum::<u64>() / metrics.len() as u64;

    println!("Performance Summary:");
    println!("Average CPU Usage: {:.2}%", avg_cpu);
    println!("Average Memory Usage: {} bytes", avg_memory);
    println!("Average Disk Usage: {} bytes", avg_disk);
}
