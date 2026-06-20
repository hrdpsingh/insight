use sysinfo::{ProcessesToUpdate, System};

use crate::metrics;

pub struct Probe {
    pub system: System,
    pub cpu: Cpu,
    pub memory: Memory,
    pub swap: Swap,
    pub processes: Vec<Process>,
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

pub struct Swap {
    pub used: u64,
    pub total: u64,
}

pub struct Process {
    pub pid: u32,
    pub name: String,
    pub memory: u64,
    pub cpu: f32,
}

impl Default for Probe {
    fn default() -> Self {
        let mut system = System::new_all();
        system.refresh_processes(ProcessesToUpdate::All, true);
        let processes = metrics::get_processes(&system);

        Self {
            cpu: Cpu {
                name: system
                    .cpus()
                    .first()
                    .map(|cpu| cpu.brand().to_string())
                    .unwrap_or_else(|| "Unavailable".to_string()),
                architecture: System::cpu_arch().to_string(),
                core_count: system.cpus().len(),
                history: vec![0.0; 60],
            },
            memory: Memory {
                used: system.used_memory(),
                total: system.total_memory(),
            },
            swap: Swap {
                used: system.used_swap(),
                total: system.total_swap(),
            },
            processes,
            system,
        }
    }
}
