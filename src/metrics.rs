use chrono::Local;
use sysinfo::{
    CpuRefreshKind, DiskRefreshKind, Disks, MemoryRefreshKind, Networks, ProcessRefreshKind,
    RefreshKind, System,
};

use crate::state::{Cpu, Disk, Insight, Mode, Network, Process, Processes};

pub fn initialize() -> Insight {
    let system = System::new_with_specifics(refresh_system());
    let disks = Disks::new_with_refreshed_list_specifics(DiskRefreshKind::nothing().with_storage());
    let networks = Networks::new_with_refreshed_list();

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
        memory: crate::state::Memory { used: 0, total: 0 },
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
        network: Network {
            interfaces: networks,
            incoming: 0,
            outgoing: 0,
            receiving: false,
            sending: false,
            received: 0,
            sent: 0,
        },
        mode: Mode::default(),
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
    insight.storage.time = Local::now().format("%H:%M:%S").to_string();
    insight.disks.refresh_specifics(false, refresh_disks());
    insight.storage.disks.clear();

    insight
        .storage
        .disks
        .extend(insight.disks.list().iter().map(|disk| Disk {
            total: disk.total_space(),
            free: disk.available_space(),
        }));
}

pub fn update_network(insight: &mut Insight) {
    insight.network.interfaces.refresh(true);

    insight.network.incoming = insight
        .network
        .interfaces
        .values()
        .map(|data| data.received())
        .sum();

    insight.network.outgoing = insight
        .network
        .interfaces
        .values()
        .map(|data| data.transmitted())
        .sum();

    insight.network.receiving = insight.network.incoming != 0;
    insight.network.sending = insight.network.outgoing != 0;

    insight.network.received += insight.network.incoming;
    insight.network.sent += insight.network.outgoing;
}

pub fn format_bytes(bytes: u64) -> String {
    let units = ["B", "KiB", "MiB", "GiB"];
    let mut index = 0;
    let mut number = bytes as f64;

    while number >= 1024.0 && index < 3 {
        number /= 1024.0;
        index += 1;
    }

    if index == 0 {
        format!("{:.0} {}", number, units[index])
    } else {
        format!("{:.1} {}", number, units[index])
    }
}
