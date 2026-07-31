use crate::state::{Insight, Process};
use chrono::Local;
use sysinfo::{
    CpuRefreshKind, DiskRefreshKind, MemoryRefreshKind, ProcessRefreshKind, RefreshKind,
};

pub fn refresh_system() -> RefreshKind {
    RefreshKind::nothing()
        .with_cpu(CpuRefreshKind::nothing().with_cpu_usage())
        .with_memory(MemoryRefreshKind::nothing().with_ram())
        .with_processes(ProcessRefreshKind::nothing().with_memory().with_cpu())
}

pub fn refresh_disks() -> DiskRefreshKind {
    DiskRefreshKind::nothing().with_storage()
}

pub fn update_cpu(insight: &mut Insight) {
    let cpus = insight.system.cpus();
    let average = if cpus.is_empty() {
        0.0
    } else {
        cpus.iter().map(sysinfo::Cpu::cpu_usage).sum::<f32>() / cpus.len() as f32
    };

    insight.cpu.history.pop_front();
    insight.cpu.history.push_back(average);
}

pub fn update_memory(insight: &mut Insight) {
    insight.memory.used = insight.system.used_memory();
    insight.memory.total = insight.system.total_memory();
}

pub fn update_processes(insight: &mut Insight) {
    insight.processes.list.clear();

    insight
        .processes
        .list
        .extend(
            insight
                .system
                .processes()
                .iter()
                .map(|(pid, process)| Process {
                    pid: pid.as_u32(),
                    name: process.name().to_string_lossy().to_string(),
                    memory: process.memory(),
                }),
        );

    insight
        .processes
        .list
        .sort_by_key(|p2| std::cmp::Reverse(p2.memory));

    let pages = insight
        .processes
        .list
        .len()
        .div_ceil(crate::constant::process::COUNT)
        .max(1);
    if insight.processes.page > pages {
        insight.processes.page = pages;
    }
}

pub fn update_storage(insight: &mut Insight) {
    insight
        .disks
        .refresh_specifics(false, DiskRefreshKind::nothing().with_storage());

    insight.storage.total = insight
        .disks
        .list()
        .iter()
        .map(sysinfo::Disk::total_space)
        .sum();

    let free: u64 = insight
        .disks
        .list()
        .iter()
        .map(sysinfo::Disk::available_space)
        .sum();

    insight.storage.used = insight.storage.total - free;
    insight.storage.time = Local::now().format("%H:%M:%S").to_string();
}

pub fn update_network(insight: &mut Insight) {
    insight.networks.refresh(true);

    insight.network.incoming = insight
        .networks
        .values()
        .map(sysinfo::NetworkData::received)
        .sum();

    insight.network.outgoing = insight
        .networks
        .values()
        .map(sysinfo::NetworkData::transmitted)
        .sum();

    insight.network.receiving = insight.network.incoming != 0;
    insight.network.sending = insight.network.outgoing != 0;

    insight.network.received += insight.network.incoming;
    insight.network.sent += insight.network.outgoing;
}
