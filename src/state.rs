use crate::metrics;
use sysinfo::{Disks, System};

pub struct Insight {
    pub cpu: Cpu,
    pub memory: Memory,
    pub processes: Processes,
    pub storage: Storage,
    pub system: System,
    pub disks: Disks,
}

pub struct Cpu {
    pub name: String,
    pub architecture: String,
    pub core_count: usize,
    pub history: Vec<f32>,
}

pub struct Memory {
    pub used: f32,
    pub total: f32,
}

pub struct Processes {
    pub list: Vec<Process>,
    pub page: usize,
}

pub struct Process {
    pub pid: u32,
    pub name: String,
    pub memory: f32,
}

pub struct Storage {
    pub disks: Vec<Disk>,
    pub time: String,
}

pub struct Disk {
    pub total: f32,
    pub available: f32,
}

impl Default for Insight {
    fn default() -> Self {
        metrics::initialize()
    }
}
