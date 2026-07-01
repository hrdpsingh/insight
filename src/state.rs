use sysinfo::{Disks, System};

pub struct Insight {
    pub cpu: Cpu,
    pub memory: Memory,
    pub processes: Vec<Process>,
    pub page: usize,
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
    pub used: u64,
    pub total: u64,
}

pub struct Process {
    pub pid: u32,
    pub name: String,
    pub memory: u64,
}

pub struct Storage {
    pub disks: Vec<Disk>,
}
pub struct Disk {
    pub total: u64,
    pub available: u64,
}

impl Default for Insight {
    fn default() -> Self {
        let system = System::new_all();
        let disks = Disks::new_with_refreshed_list();

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
            processes: system
                .processes()
                .iter()
                .map(|(pid, process)| Process {
                    pid: pid.as_u32(),
                    name: process.name().to_string_lossy().to_string(),
                    memory: process.memory(),
                })
                .collect(),
            page: 1,
            storage: Storage {
                disks: disks
                    .list()
                    .iter()
                    .map(|disk| Disk {
                        total: disk.total_space(),
                        available: disk.available_space(),
                    })
                    .collect(),
            },
            system,
            disks,
        }
    }
}
