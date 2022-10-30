use std::fmt::Display;





pub enum CargoCleanerError {
    ReadDir,
    ReadCargoToml,
    CountDirSize,
    SpawnCargoCleanCommand
}

impl Display for CargoCleanerError{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self{
            CargoCleanerError::ReadDir => "Reading dir",
            CargoCleanerError::ReadCargoToml => "Reading Cargo.toml file",
            CargoCleanerError::CountDirSize => "Counting dir size",
            CargoCleanerError::SpawnCargoCleanCommand => r#"Spawning "cargo clean" command"#,
        })
    }
}


