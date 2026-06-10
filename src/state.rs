use sysinfo::System;

pub struct Probe {
    pub system: System,
    pub processor: Processor,
}

pub struct Processor {
    pub name: String,
    pub architecture: String,
    pub core_count: usize,
    pub usage_history: Vec<f32>,
}

impl Default for Probe {
    fn default() -> Self {
        let system = System::new_all();
        let name: String = system
            .cpus()
            .first()
            .map(|cpu| cpu.brand().to_string())
            .unwrap_or_else(|| "Unavailable".to_string());
        let architecture = System::cpu_arch().to_string();
        let core_count = system.cpus().len();
        let usage_history = vec![0.0; 60];


        Self {
            system,
            processor: Processor {
                name,
                architecture,
                core_count,
                usage_history,
            },
        }
    }
}