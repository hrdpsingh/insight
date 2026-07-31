use crate::state::{Configuration, Mode};
use directories::ProjectDirs;
use std::{fs, path::PathBuf};

fn configuration_path() -> Result<PathBuf, Box<dyn std::error::Error>> {
    let project_directories = ProjectDirs::from("", "", "Insight")
        .ok_or("Could not determine configuration directory")?;
    let configuration_directory = project_directories.config_dir();
    fs::create_dir_all(configuration_directory)?;
    Ok(configuration_directory.join("configuration.toml"))
}

pub fn load_configuration() -> Result<Configuration, Box<dyn std::error::Error>> {
    let path = configuration_path()?;
    let string = std::fs::read_to_string(path)?;
    Ok(toml::from_str(&string)?)
}

pub fn save_configuration(mode: &Mode) -> Result<(), Box<dyn std::error::Error>> {
    let path = configuration_path()?;
    let configuration = Configuration { mode: mode.clone() };
    let toml = toml::to_string_pretty(&configuration)?;
    std::fs::write(path, toml)?;
    Ok(())
}

pub fn format_bytes(bytes: u64) -> String {
    let units = ["B", "KiB", "MiB", "GiB", "TiB"];
    let mut index = 0;
    let mut number = bytes as f64;

    while number >= 1024.0 && index < units.len() - 1 {
        number /= 1024.0;
        index += 1;
    }

    if index == 0 {
        format!("{:.0} {}", number, units[index])
    } else {
        format!("{:.1} {}", number, units[index])
    }
}
