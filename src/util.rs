use anyhow::{Context, Result};
use std::fs::{self, File};
use std::io::Write;
use std::path::Path;
use std::process::{Command, Stdio};

pub fn write<P: AsRef<Path>>(path: P, content: impl AsRef<[u8]>) -> Result<()> {
    if let Some(parent) = path.as_ref().parent() {
        fs::create_dir_all(parent)?;
    }
    let mut f = File::create(path)?;
    f.write_all(content.as_ref())?;
    Ok(())
}

/// Run a command for side effects, erroring on non-zero status.
pub fn run(cmd: &str, args: &[&str], cwd: &Path) -> Result<()> {
    let status = Command::new(cmd)
        .args(args)
        .current_dir(cwd)
        .stdin(Stdio::null())
        .status()
        .with_context(|| format!("failed to run `{cmd} {}`", args.join(" ")))?;
    if !status.success() {
        anyhow::bail!("command `{cmd}` failed with status {status}");
    }
    Ok(())
}

/// Find the system Python version or return a default.
pub fn detect_system_python() -> String {
    let candidate = which::which("python3")
        .or_else(|_| which::which("python"))
        .ok();
    if let Some(bin) = candidate {
        if let Ok(out) = Command::new(bin)
            .arg("-c")
            .arg("import sys;print('.'.join(map(str, sys.version_info[:3])))")
            .output()
        {
            let s = String::from_utf8_lossy(&out.stdout).trim().to_string();
            if !s.is_empty() {
                return s;
            }
        }
    }
    "3.11.0".to_string()
}
