use crate::state::{Probe, Process};

pub fn update_cpu_usage(probe: &mut Probe) {
    let cpus = probe.system.cpus();

    let average = if cpus.is_empty() {
        0.0
    } else {
        cpus.iter().map(|cpu| cpu.cpu_usage()).sum::<f32>() / cpus.len() as f32
    };

    probe.cpu.history.remove(0);
    probe.cpu.history.push(average);
}

pub fn update_memory_usage(probe: &mut Probe) {
    probe.memory.used = probe.system.used_memory();
    probe.memory.total = probe.system.total_memory();
}

pub fn update_swap_usage(probe: &mut Probe) {
    probe.swap.used = probe.system.used_swap();
    probe.swap.total = probe.system.total_swap();
}

pub fn get_processes(probe: &mut Probe) {
    let mut processes: Vec<Process> = probe
        .system
        .processes()
        .iter()
        .map(|(pid, process)| Process {
            pid: pid.as_u32(),
            name: process.name().to_string_lossy().to_string(),
            memory: process.memory() / (1024 * 1024),
        })
        .collect();
    processes.sort_by_key(|process| std::cmp::Reverse(process.memory));
    probe.processes = processes;
}
