use crate::app::Probe;

pub fn update_cpu_usage(probe: &mut Probe) {
    probe.system.refresh_cpu_all();
    let cpu_usage = probe.system.global_cpu_usage();
    probe.cpu_usage_history.remove(0);
    probe.cpu_usage_history.push(cpu_usage);
}
