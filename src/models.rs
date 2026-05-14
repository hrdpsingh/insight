#[derive(Clone, Copy, Default, PartialEq)]
pub enum Page {
    #[default]
    Software,
    Hardware,
    Firmware,
}
