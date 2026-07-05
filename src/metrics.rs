use chrono::Local;
use sysinfo::{
    CpuRefreshKind, DiskRefreshKind, Disks, MemoryRefreshKind, ProcessRefreshKind, RefreshKind,
    System,
};

use crate::state::{Cpu, Disk, Insight, Process, Processes};

pub fn initialize() -> Insight {
    let system = System::new_with_specifics(refresh_system());
    let disks = Disks::new_with_refreshed_list_specifics(DiskRefreshKind::nothing().with_storage());

    let mut insight = Insight {
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
        memory: crate::state::Memory {
            used: 0.0,
            total: 0.0,
        },
        processes: Processes {
            list: Vec::new(),
            page: 1,
        },
        storage: crate::state::Storage {
            disks: Vec::new(),
            time: "Unavailable".to_string(),
        },
        system,
        disks,
    };

    update_memory(&mut insight);
    update_processes(&mut insight);
    update_storage(&mut insight);

    insight
}

pub fn refresh_system() -> RefreshKind {
    RefreshKind::nothing()
        .with_cpu(CpuRefreshKind::nothing().with_cpu_usage())
        .with_memory(MemoryRefreshKind::nothing().with_ram())
        .with_processes(ProcessRefreshKind::nothing().with_memory())
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
    insight.memory.used = to_gb(insight.system.used_memory());
    insight.memory.total = to_gb(insight.system.total_memory());
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
                    memory: to_mb(process.memory()),
                }),
        );
    insight
        .processes
        .list
        .sort_by(|p1, p2| p2.memory.total_cmp(&p1.memory));
}

pub fn update_storage(insight: &mut Insight) {
    insight.storage.time = Local::now().format("%H:%M:%S").to_string();
    insight.disks.refresh_specifics(false, refresh_disks());
    insight.storage.disks.clear();

    insight
        .storage
        .disks
        .extend(insight.disks.list().iter().map(|disk| Disk {
            total: to_gb(disk.total_space()),
            available: to_gb(disk.available_space()),
        }));
}

fn to_gb(bytes: u64) -> f32 {
    bytes as f32 / (1024 * 1024 * 1024) as f32
}

fn to_mb(bytes: u64) -> f32 {
    bytes as f32 / (1024 * 1024) as f32
}
