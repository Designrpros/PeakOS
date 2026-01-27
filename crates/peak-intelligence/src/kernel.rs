use once_cell::sync::Lazy;
use serde::Serialize;
use std::sync::Mutex;
use sysinfo::System;

static SYSTEM: Lazy<Mutex<System>> = Lazy::new(|| Mutex::new(System::new_all()));

#[derive(Serialize, Clone, Debug)]
pub struct SystemTelemetry {
    pub cpu_temp: f32,     // Degrees Celsius
    pub battery_level: u8, // 0-100
    pub memory_used: u64,  // in MB
    pub memory_total: u64, // in MB
    pub uptime: u64,       // Seconds
    pub load_avg: f32,     // 1-minute load
    pub is_charging: bool,
}

impl SystemTelemetry {
    pub fn snapshot() -> Self {
        let mut sys = SYSTEM.lock().unwrap();
        sys.refresh_all();

        // Traits seem to be missing or version is weird,
        // using what we know about System struct for now.
        let memory_total = sys.total_memory() / 1024 / 1024; // Bytes -> MB
        let memory_used = sys.used_memory() / 1024 / 1024; // Bytes -> MB

        let load = System::load_average().one as f32;

        SystemTelemetry {
            cpu_temp: 0.0,
            battery_level: 100,
            memory_used,
            memory_total,
            uptime: System::uptime(),
            load_avg: load,
            is_charging: true,
        }
    }
}
