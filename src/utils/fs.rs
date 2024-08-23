use std::fs;
use std::io::{self, BufRead};
use std::path::Path;

pub fn read_lines<P: AsRef<Path>>(path: P) -> anyhow::Result<io::Lines<io::BufReader<fs::File>>> {
    let fp = fs::File::open(path)?;
    Ok(io::BufReader::new(fp).lines())
}

#[cfg(unix)]
pub fn set_executable<P: AsRef<Path>>(path: P) -> anyhow::Result<()> {
    use std::os::unix::fs::PermissionsExt;
    fs::set_permissions(&path, fs::Permissions::from_mode(0o755))?;
    Ok(())
}

#[cfg(not(unix))]
pub fn set_executable<P: AsRef<Path>>(_path: P) -> anyhow::Result<()> {
    Ok(())
}
