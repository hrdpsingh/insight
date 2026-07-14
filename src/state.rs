use iced::Theme;
use sysinfo::{Disks, Networks, System};

pub struct Insight {
    pub cpu: Cpu,
    pub memory: Memory,
    pub processes: Processes,
    pub storage: Storage,
    pub network: Network,
    pub system: System,
    pub disks: Disks,
    pub networks: Networks,
    pub theme: Theme,
}

pub struct Cpu {
    pub name: String,
    pub architecture: String,
    pub core_count: usize,
    pub history: Vec<f32>,
}

pub struct Memory {
    pub used: u64,
    pub total: u64,
}

pub struct Processes {
    pub list: Vec<Process>,
    pub page: usize,
}

pub struct Process {
    pub pid: u32,
    pub name: String,
    pub memory: u64,
}

pub struct Storage {
    pub total: u64,
    pub used: u64,
    pub time: String,
}

pub struct Network {
    pub outgoing: u64,
    pub incoming: u64,
    pub receiving: bool,
    pub sending: bool,
    pub received: u64,
    pub sent: u64,
}
