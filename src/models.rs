#[derive(Clone, Copy, Default, PartialEq)]
pub enum Page {
    #[default]
    Cpu,
    Software,
    Firmware,
}
