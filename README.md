# Rust System Performance Profiler

This is a simple system performance profiler written in Rust, designed to collect and report on various system metrics on Linux systems.

## Features

- Collects CPU usage, memory usage, and disk usage metrics
- Stores collected data in a JSON file
- Provides real-time snapshots and summary reports

## Requirements

- Rust (latest stable version)
- Linux operating system

## Installation

1. Clone this repository:
   ```
   git clone https://github.com/yourusername/rust-system-profiler.git
   cd rust-system-profiler
   ```

2. Build the project:
   ```
   cargo build --release
   ```

## Usage

Run the profiler with default settings (5-second interval for 60 seconds):

```
cargo run --release
```

Or specify custom interval and duration:

```
cargo run --release -- --interval 10 --duration 120
```

This will run the profiler every 10 seconds for a total of 120 seconds.

## Output

The profiler will print real-time snapshots to the console and store the data in a file named `profile_data.json` in the current directory. After completion, it will print a summary of the collected data.

## License

This project is open-source and available under the MIT License.
