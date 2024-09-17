use super::{SystemProfiler, SystemMetrics};
use sysinfo::{System, SystemExt, CpuExt};
use chrono::Utc;

pub struct LinuxProfiler {
    system: System,
}

impl SystemProfiler for LinuxProfiler {
    fn new() -> Self {
        LinuxProfiler {
            system: System::new_all(),
        }
    }

    fn collect_metrics(&self) -> SystemMetrics {
        self.system.refresh_all();

        let cpu_usage = self.system.global_cpu_info().cpu_usage();
        let memory_usage = self.system.used_memory();
        let disk_usage = self.system.disks().iter().map(|disk| disk.total_space() - disk.available_space()).sum();

        SystemMetrics {
            timestamp: Utc::now(),
            cpu_usage,
            memory_usage,
            disk_usage,
        }
    }
}
