use crate::app::Probe;

pub fn update_cpu_usage(probe: &mut Probe) {
    probe.system.refresh_cpu_all();
    let cpus = probe.system.cpus();

    for (i, cpu) in cpus.iter().enumerate() {
        let usage = cpu.cpu_usage();
        probe.cpu_usage_history[i].remove(0);
        probe.cpu_usage_history[i].push(usage);
    }
}
