use sysinfo::{ProcessesToUpdate, System};

pub struct Probe {
    pub system: System,
    pub cpu: Cpu,
    pub processes: Processes,
}

pub struct Cpu {
    pub name: String,
    pub architecture: String,
    pub core_count: usize,
    pub history: Vec<f32>,
}

pub struct Processes {
    pub entries: Vec<Process>,
}

pub struct Process {
    pub pid: u32,
    pub name: String,
    pub memory_kb: u64,
}

impl Default for Probe {
    fn default() -> Self {
        let mut system = System::new_all();
        system.refresh_processes(ProcessesToUpdate::All, true);

        let processes = Processes {
            entries: collect_processes(&system),
        };

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
            processes,
            system,
        }
    }
}

pub fn collect_processes(system: &System) -> Vec<Process> {
    let mut entries: Vec<Process> = system
        .processes()
        .iter()
        .map(|(pid, process)| Process {
            pid: pid.as_u32(),
            name: process.name().to_string_lossy().to_string(),
            memory_kb: process.memory() / 1024,
        })
        .collect();

    entries.sort_by_key(|b| std::cmp::Reverse(b.memory_kb));
    entries
}
