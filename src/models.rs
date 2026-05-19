#[derive(Clone, Copy, Default, PartialEq)]
pub enum Page {
    #[default]
    Software,
    Cpu,
    Firmware,
}
