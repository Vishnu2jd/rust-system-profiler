use super::Storage;
use crate::profiler::SystemMetrics;
use std::fs::{File, OpenOptions};
use std::io::{Write, Read};
use serde_json;

pub struct FileStorage {
    path: String,
}

impl Storage for FileStorage {
    fn new(path: &str) -> Self {
        FileStorage {
            path: path.to_string(),
        }
    }

    fn store(&self, metrics: &SystemMetrics) {
        let json = serde_json::to_string(&metrics).unwrap();
        let mut file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(&self.path)
            .unwrap();
        writeln!(file, "{}", json).unwrap();
    }

    fn load_all(&self) -> Vec<SystemMetrics> {
        let mut file = File::open(&self.path).unwrap();
        let mut contents = String::new();
        file.read_to_string(&mut contents).unwrap();

        contents.lines()
            .map(|line| serde_json::from_str(line).unwrap())
            .collect()
    }
}
