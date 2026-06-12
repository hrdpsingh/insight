use crate::state::{Probe, Process};
use sysinfo::System;

pub fn update_cpu_usage(probe: &mut Probe) {
    probe.system.refresh_cpu_all();
    let cpus = probe.system.cpus();

    let average = if cpus.is_empty() {
        0.0
    } else {
        cpus.iter().map(|cpu| cpu.cpu_usage()).sum::<f32>() / cpus.len() as f32
    };

    probe.cpu.history.remove(0);
    probe.cpu.history.push(average);
}

pub fn get_processes(system: &System) -> Vec<Process> {
    let mut processes: Vec<Process> = system
        .processes()
        .iter()
        .map(|(pid, process)| Process {
            pid: pid.as_u32(),
            name: process.name().to_string_lossy().to_string(),
            memory: process.memory() / (1024 * 1024),
        })
        .collect();
    processes.sort_by_key(|process| std::cmp::Reverse(process.memory));
    processes
}
