use crate::palette::Palette;
use iced::Theme;
use sysinfo::{Disks, Networks, System};

pub struct Insight {
    pub cpu: Cpu,
    pub memory: Memory,
    pub processes: Processes,
    pub storage: Storage,
    pub system: System,
    pub disks: Disks,
    pub network: Network,
    pub mode: Mode,
}

pub struct Cpu {
    pub name: String,
    pub architecture: String,
    pub core_count: usize,
    pub history: Vec<f32>,
}

pub struct Memory {
    pub used: u64,
    pub total: u64,
}

pub struct Processes {
    pub list: Vec<Process>,
    pub page: usize,
}

pub struct Process {
    pub pid: u32,
    pub name: String,
    pub memory: u64,
    pub cpu: f32,
}

pub struct Storage {
    pub disks: Vec<Disk>,
    pub time: String,
}

pub struct Disk {
    pub total: u64,
    pub free: u64,
}

pub struct Network {
    pub interfaces: Networks,
    pub outgoing: u64,
    pub incoming: u64,
    pub receiving: bool,
    pub sending: bool,
    pub received: u64,
    pub sent: u64,
}

#[derive(Default, Clone)]
pub enum Mode {
    #[default]
    Light,
    Dark,
}

impl Mode {
    pub fn convert(&self) -> Theme {
        match self {
            Mode::Light => Theme::Light,
            Mode::Dark => Theme::Dark,
        }
    }
}

pub trait ExtendTheme {
    fn custom(&self) -> &'static Palette;
}

impl ExtendTheme for Theme {
    fn custom(&self) -> &'static Palette {
        match self {
            Theme::Dark => &Palette::DARK,
            _ => &Palette::LIGHT,
        }
    }
}
