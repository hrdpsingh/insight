use crate::state::{Disk, Insight, Process};

pub fn update_cpu_usage(insight: &mut Insight) {
    let cpus = insight.system.cpus();

    let average = if cpus.is_empty() {
        0.0
    } else {
        cpus.iter().map(|cpu| cpu.cpu_usage()).sum::<f32>() / cpus.len() as f32
    };

    insight.cpu.history.remove(0);
    insight.cpu.history.push(average);
}

pub fn update_memory_usage(insight: &mut Insight) {
    insight.memory.used = insight.system.used_memory();
    insight.memory.total = insight.system.total_memory();
}

pub fn get_processes(insight: &mut Insight) {
    let mut processes: Vec<Process> = insight
        .system
        .processes()
        .iter()
        .map(|(pid, process)| Process {
            pid: pid.as_u32(),
            name: process.name().to_string_lossy().to_string(),
            memory: process.memory(),
        })
        .collect();
    processes.sort_by_key(|process| std::cmp::Reverse(process.memory));
    insight.processes = processes;
}

pub fn update_storage(insight: &mut Insight) {
    insight.disks.refresh(true);

    insight.storage.disks = insight
        .disks
        .list()
        .iter()
        .map(|disk| Disk {
            total: disk.total_space(),
            available: disk.available_space(),
        })
        .collect();
}
