use std::fs::File;
use std::io::{Read, Write};
use std::process::Command;
use std::path::{Path, PathBuf};
use tempfile::NamedTempFile;

pub fn labelg(graph6: String) -> Result<String, String> {
    let current_dir = std::env::current_dir()
        .map_err(|_| "labelg-rust: failed to get current dir".to_string())?;
    let labelg_path: PathBuf = [
        &current_dir,
        Path::new("bin"),
        Path::new("labelg")
    ].iter().collect();

    let mut in_file = NamedTempFile::new()
        .map_err(|_| "labelg-rust: failed to create temp input file".to_string())?;

    let mut out_file = NamedTempFile::new()
        .map_err(|_| "labelg-rust: failed to create temp output file".to_string())?;

    writeln!(in_file, "{}", graph6).
        map_err(|_| "labelg-rust: failed to write to temp input file".to_string())?;

    let status = Command::new(labelg_path)
        .arg(in_file.path())
        .arg(out_file.path())
        .stdout(std::process::Stdio::null())
        .stderr(std::process::Stdio::null())
        .status()
        .map_err(|_| "labelg-rust: failed to run labelg".to_string())?;

    if !status.success() {
        return Err("labelg-rust: labelg exited with status 1".to_string());
    }
    
    let mut out_file = out_file.reopen()
        .map_err(|_| "labelg-rust: failed to reopen temp output file".to_string())?;

    let mut output = String::new();
    out_file.read_to_string(&mut output)
        .map_err(|_| "labelg-rust: failed to read the file to string".to_string())?;

    Ok(output)

}
