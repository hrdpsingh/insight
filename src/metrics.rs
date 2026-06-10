use crate::state::Probe;

pub fn update_cpu_usage(probe: &mut Probe) {
    probe.system.refresh_cpu_all();
    let cpus = probe.system.cpus();

    let average = if cpus.is_empty() {
        0.0
    } else {
        cpus.iter().map(|cpu| cpu.cpu_usage()).sum::<f32>() / cpus.len() as f32
    };

    probe.processor.usage_history.remove(0);
    probe.processor.usage_history.push(average);
}