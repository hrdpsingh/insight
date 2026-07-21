use crate::state::{Insight, Process};
use chrono::Local;
use sysinfo::DiskRefreshKind;
use sysinfo::{CpuRefreshKind, MemoryRefreshKind, ProcessRefreshKind, RefreshKind};

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
        cpus.iter().map(|cpu| cpu.cpu_usage()).sum::<f32>() / cpus.len() as f32
    };

    insight.cpu.history.remove(0);
    insight.cpu.history.push(average);
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
}

pub fn update_storage(insight: &mut Insight) {
    insight
        .disks
        .refresh_specifics(false, DiskRefreshKind::nothing().with_storage());

    insight.storage.total = insight
        .disks
        .list()
        .iter()
        .map(|disk| disk.total_space())
        .sum();

    let free: u64 = insight
        .disks
        .list()
        .iter()
        .map(|disk| disk.available_space())
        .sum();

    insight.storage.used = insight.storage.total - free;
    insight.storage.time = Local::now().format("%H:%M:%S").to_string();
}

pub fn update_network(insight: &mut Insight) {
    insight.networks.refresh(true);

    insight.network.incoming = insight.networks.values().map(|data| data.received()).sum();

    insight.network.outgoing = insight
        .networks
        .values()
        .map(|data| data.transmitted())
        .sum();

    insight.network.receiving = insight.network.incoming != 0;
    insight.network.sending = insight.network.outgoing != 0;

    insight.network.received += insight.network.incoming;
    insight.network.sent += insight.network.outgoing;
}

pub fn format_bytes(bytes: u64) -> String {
    let units = ["B", "KiB", "MiB", "GiB", "TiB"];
    let mut index = 0;
    let mut number = bytes as f64;

    while number >= 1024.0 && index < 4 {
        number /= 1024.0;
        index += 1;
    }

    if index == 0 {
        format!("{:.0} {}", number, units[index])
    } else {
        format!("{:.1} {}", number, units[index])
    }
}
