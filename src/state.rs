pub struct Probe {
    pub page: Page,
    pub cpu_usage_history: Vec<f32>,
}

impl Default for Probe {
    fn default() -> Self {
        Self {
            page: Page::default(),
            cpu_usage_history: vec![0.0; 60],
        }
    }
}

#[derive(Clone, Copy, Default, PartialEq)]
pub enum Page {
    #[default]
    Software,
    Hardware,
    Firmware,
}
