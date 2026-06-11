use crate::state::{Probe, collect_processes};
use sysinfo::ProcessesToUpdate;

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

pub fn update_processes(probe: &mut Probe) {
    probe.system.refresh_processes(ProcessesToUpdate::All, true);
    probe.processes.entries = collect_processes(&probe.system);
}
