use std::env;
use std::ffi::OsStr;
use std::fs::OpenOptions;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::io::Write;

#[derive(Debug)]
enum OS {
    Windows,
    MacOS,
    Linux,
}

#[derive(Debug)]
pub struct EnvVar {
    pub name: String,
    pub value: String,
}

pub struct EnvironmentVariables;

impl EnvironmentVariables {
    pub fn update(env_vars: &[EnvVar]) -> Result<(), Box<dyn std::error::Error>> {
        let os = detect_os()?;
        match os {
            OS::Windows => update_windows(env_vars),
            OS::Linux => update_linux(env_vars),
            OS::MacOS => update_mac(env_vars)
        }
    }
}

fn detect_os() -> Result<OS, Box<dyn std::error::Error>> {
    let os = std::env::consts::OS;
    match os {
        "windows" => Ok(OS::Windows),
        "macos" => Ok(OS::MacOS),
        "linux" => Ok(OS::Linux),
        _ => panic!("Unsupported OS: {}", os),
    }
}

fn update_windows(env_vars: &[EnvVar]) -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::new("cmd");
    cmd.args(&["/C"]);
    for var in env_vars {
        cmd.arg(format!("setx {} {}", var.name, var.value));
    }
    let output = cmd.output().map_err(|e| format!("Failed to execute command: {}", e))?;
    if output.status.success() {
        Ok(())
    } else {
        panic!("Problem updating environment variable: {}", output.stderr[0])
    }
}

fn get_config_path() -> Result<PathBuf, Box<dyn std::error::Error>> {
    // Get the user's default shell
    let shell = env::var("SHELL")?;
    let shell_str = shell.trim();

    // Determine the shell configuration file based on the shell
    let config_path = match Path::new(shell_str).file_name().and_then(OsStr::to_str) {
        Some("bash") => ".bashrc",
        Some("zsh") => ".zshrc",
        _ => panic!("Unsupported shell environment detected!")
    };
    Ok(PathBuf::from(config_path))
}

fn update_linux(env_vars: &[EnvVar]) -> Result<(), Box<dyn std::error::Error>> {
    let config_file = get_config_path()?;
    update_unix(env_vars, config_file)
}

fn update_mac(env_vars: &[EnvVar]) -> Result<(), Box<dyn std::error::Error>> {
    let config_file = get_config_path()?;

    update_unix(env_vars, config_file)
}

fn update_unix(env_vars: &[EnvVar], config_file: PathBuf) -> Result<(), Box<dyn std::error::Error>> {
    let home_dir = env::var("HOME").map_err(|e| format!("Failed to get HOME directory: {}", e))?;
    let config_path = Path::new(&home_dir).join(config_file);
    let mut config_file = OpenOptions::new()
        .append(true)
        .create(true)
        .open(&config_path)
        .map_err(|e| format!("Failed to open {}: {}", config_path.display(), e))?;

    for var in env_vars {
        writeln!(&mut config_file, "export {}={}", var.name, var.value)
            .map_err(|e| format!("Failed to write environment variable to {}: {}", config_path.display(), e))?;
    }

    let output = Command::new("sh")
        .arg("-c")
        .arg(format!("source {}", config_path.to_str().unwrap()))
        .output()
        .map_err(|e| format!("Failed to execute command: {}", e))?;

    if output.status.success() {
        Ok(())
    } else {
        Err(format!("Problem updating environment variable: {}", output.stderr[0]).into())
    }
}

// Add the test module at the end of your implementation file
#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::{File, read_to_string};
    use tempfile::NamedTempFile;

    #[test]
    fn test_get_config_path() {
        match get_config_path() {
            Ok(config_path) => {
                assert!(
                    config_path == PathBuf::from(".bashrc") || config_path == PathBuf::from(".zshrc"),
                    "Unexpected config path: {}",
                    PathBuf::from(config_path).to_string_lossy()
                );
            }
            Err(e) => panic!("Error: {}", e),
        }
    }

    #[test]
    fn test_update_unix() {
        let temp_config = NamedTempFile::new().expect("Failed to create temporary file");
        let temp_config_path = temp_config.path().to_path_buf();

        let env_vars = [
            EnvVar {
                name: "TEST_VARIABLE_1".to_string(),
                value: "VALUE1".to_string(),
            },
            EnvVar {
                name: "TEST_VARIABLE_2".to_string(),
                value: "VALUE2".to_string(),
            },
        ];

        update_unix(&env_vars, temp_config_path.clone())
            .expect("Failed to update temporary shell config file");

        let config_contents = read_to_string(temp_config_path)
            .expect("Failed to read temporary shell config file");

        assert!(
            config_contents.contains("export TEST_VARIABLE_1=VALUE1"),
            "Expected environment variable not found in temporary shell config file"
        );
        assert!(
            config_contents.contains("export TEST_VARIABLE_2=VALUE2"),
            "Expected environment variable not found in temporary shell config file"
        );
    }
}