use std::{env, fs};
use std::error::Error;
use std::ffi::OsStr;
use std::fs::OpenOptions;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::io::Write;

use dirs::home_dir;

#[derive(Debug)]
pub struct EnvVar {
    pub name: String,
    pub value: String,
}


pub trait EnvironmentVariableHandler {
    fn exists(&self, name: &str) -> Result<bool, Box<dyn Error>>;
    fn get(&self, name: &str) -> Result<String, Box<dyn Error>>;
    fn update(&self, env_vars: &[EnvVar]) -> Result<(), Box<dyn Error>>;
}

#[cfg(windows)]
pub struct WindowsEnvironmentVariableHandler;

#[cfg(target_os = "linux")]
pub struct LinuxEnvironmentVariableHandler;

#[cfg(target_os = "macos")]
pub struct MacOSEnvironmentVariableHandler;

#[cfg(windows)]
impl EnvironmentVariableHandler for WindowsEnvironmentVariableHandler {
    fn exists(&self, name: &str) -> Result<bool, Box<dyn Error>> {
        // Windows implementation for checking the existence of an environment variable
        use winreg::enums::*;
        use winreg::RegKey;

        let hkcu = RegKey::predef(HKEY_CURRENT_USER);
        let environment = hkcu.open_subkey_with_flags("Environment", KEY_READ)?;

        match environment.get_value::<String, _>(name) {
            Ok(_) => Ok(true),
            Err(_) => Ok(false),
        }
    }

    fn get(&self, name: &str) -> Result<String, Box<dyn Error>> {
        // Windows implementation for checking the existence of an environment variable
        use winreg::enums::*;
        use winreg::RegKey;

        let hkcu = RegKey::predef(HKEY_CURRENT_USER);
        let environment = hkcu.open_subkey_with_flags("Environment", KEY_READ)?;

        Ok(environment.get_value::<String, _>(name)?)
    }

    fn update(&self, env_vars: &[EnvVar]) -> Result<(), Box<dyn Error>> {
        // Windows implementation for updating environment variables
        use winreg::enums::*;
        use winreg::RegKey;

        let hkcu = RegKey::predef(HKEY_CURRENT_USER);
        let environment = hkcu.open_subkey_with_flags("Environment", KEY_READ | KEY_WRITE)?;

        for env_var in env_vars {
            environment.set_value(&env_var.name, &env_var.value)?;
        }

        // Broadcast the WM_SETTINGCHANGE message to notify other processes of the update
        unsafe {
            use winapi::um::winuser::{SendMessageTimeoutA, HWND_BROADCAST, SMTO_ABORTIFHUNG, WM_SETTINGCHANGE};
            use std::ffi::CString;
            use std::ptr;

            let name_cstr = CString::new("Environment").unwrap();

            SendMessageTimeoutA(
                HWND_BROADCAST,
                WM_SETTINGCHANGE,
                0,
                name_cstr.as_ptr() as _,
                SMTO_ABORTIFHUNG,
                5000,
                ptr::null_mut(),
            );
        }
        Ok(())
    }
}

#[cfg(target_os = "linux")]
impl EnvironmentVariableHandler for LinuxEnvironmentVariableHandler {
    fn exists(&self, name: &str) -> Result<bool, Box<dyn Error>> {
        // Linux implementation for checking the existence of an environment variable
        match env::var(name) {
            Ok(_) => Ok(true),
            Err(_) => Ok(false)
        }
    }

    fn get(&self, name: &str) -> Result<String, Box<dyn Error>> {
        let env_var = match env::var(name) {
            Ok(key) => key,
            Err(_) => String::new()
        };
        Ok(env_var)
    }

    fn update(&self, env_vars: &[EnvVar]) -> Result<(), Box<dyn Error>> {
        let shell_info = get_config_path()?;
        let config_buf = PathBuf::from(shell_info.shell_config_file.clone());
        update_unix_shell_rc(env_vars, config_buf)?;
        update_shell(shell_info.shell_config_file.clone(), shell_info.shell_name.clone())
    }
}

#[cfg(target_os = "macos")]
impl EnvironmentVariableHandler for MacOSEnvironmentVariableHandler {
    fn exists(&self, name: &str) -> Result<bool, Box<dyn Error>> {
        // macOS implementation for checking the existence of an environment variable
        match env::var(name) {
            Ok(_) => Ok(true),
            Err(_) => Ok(false)
        }
    }

    fn get(&self, name: &str) -> Result<String, Box<dyn Error>> {
        let env_var = match env::var(name) {
            Ok(key) => key,
            Err(_) => String::new()
        };
        Ok(env_var)
    }

    fn update(&self, env_vars: &[EnvVar]) -> Result<(), Box<dyn Error>> {
        let shell_info = get_config_path()?;
        let config_buf = PathBuf::from(shell_info.shell_config_file.clone());
        update_unix_shell_rc(env_vars, config_buf)?;
        update_shell(shell_info.shell_config_file.clone(), shell_info.shell_name.clone())
    }
}

pub fn get_environment_variable_handler() -> Result<Box<dyn EnvironmentVariableHandler>, Box<dyn Error>> {
    #[cfg(windows)]
    {
        Ok(Box::new(WindowsEnvironmentVariableHandler))
    }
    #[cfg(target_os = "linux")]
    {
        Ok(Box::new(LinuxEnvironmentVariableHandler))
    }
    #[cfg(target_os = "macos")]
    {
        Ok(Box::new(MacOSEnvironmentVariableHandler))
    }
    #[cfg(not(any(windows, target_os = "linux", target_os = "macos")))]
    {
        panic!("Unsupported OS");
    }
}

#[derive(Clone, PartialEq)]
struct ShellInfo {
    pub shell_name: String,
    pub shell_config_file: String
}
impl ShellInfo {
    pub fn new(shell_name: &str, shell_config_file: &str) -> ShellInfo {
        ShellInfo {
            shell_name: shell_name.to_string(),
            shell_config_file: shell_config_file.to_string()
        }
    }
}

fn get_config_path() -> Result<ShellInfo, Box<dyn std::error::Error>> {
    // Get the user's default shell
    let shell = env::var("SHELL")?;
    let shell_str = shell.trim();

    // Determine the shell configuration file based on the shell
    let shell_info = match Path::new(shell_str).file_name().and_then(OsStr::to_str) {
        Some("bash") => ShellInfo::new("bash", ".bashrc"),
        Some("zsh") => ShellInfo::new("szh", ".zshrc"),
        _ => panic!("Unsupported shell environment detected!")
    };
    Ok(shell_info)
}

fn update_shell(config_file: String, shell_name: String) -> Result<(), Box<dyn std::error::Error>> {
    let mut config_path = match home_dir() {
        Some(path) => path,
        None => panic!("Could not find the user's home directory"),
    };
    config_path.push(config_file);

    let config_path_str = match config_path.as_path().to_str() {
        Some(path) => path,
        _ => panic!("Could not process config path")
    };

    let output = Command::new(shell_name)
        .arg("-c")
        .arg(format!("source {}", config_path_str))
        .output()
        .map_err(|e| format!("Failed to execute command: {}", e))?;

    if output.status.success() {
        Ok(())
    } else {
        panic!("Error sourcing .bashrc: {}", String::from_utf8_lossy(&output.stderr))
    }
}

fn update_unix_shell_rc(env_vars: &[EnvVar], config_file: PathBuf) -> Result<(), Box<dyn std::error::Error>> {
    let home_dir = env::var("HOME").map_err(|e| format!("Failed to get HOME directory: {}", e))?;
    let config_path = Path::new(&home_dir).join(config_file);
    
    // Read the existing content of the file
    let mut content = String::new();
    if config_path.exists() {
        content = fs::read_to_string(&config_path).map_err(|e| format!("Failed to read {}: {}", config_path.display(), e))?;
    }

    // Update the content with new environment variables
    let mut lines: Vec<String> = content.lines().map(String::from).collect();
    for var in env_vars {
        let export_line = format!("export {}=\"{}\"", var.name, var.value);
        let mut found = false;

        // Search for an existing variable and update it
        for line in lines.iter_mut() {
            if line.starts_with(&format!("export {}=", var.name)) {
                *line = export_line.clone();
                found = true;
                break;
            }
        }

        // If the variable was not found, add it
        if !found {
            lines.push(export_line);
        }
    }

    // Write the updated content back to the file
    let mut config_file = OpenOptions::new()
        .write(true)
        .truncate(true)
        .create(true)
        .open(&config_path)
        .map_err(|e| format!("Failed to open {}: {}", config_path.display(), e))?;
    
    for line in lines {
        writeln!(&mut config_file, "{}", line)
            .map_err(|e| format!("Failed to write updated content to {}: {}", config_path.display(), e))?;
    }

    Ok(())
}

// Add the test module at the end of your implementation file
#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::read_to_string;
    use tempfile::NamedTempFile;

    // #[test]
    // fn test_get_config_path() {
    //     match get_config_path() {
    //         Ok(config_path) => {
    //             assert!(
    //                 config_path == ".bashrc" || config_path == ".zshrc",
    //                 "Unexpected config path: {}",
    //                 PathBuf::from(config_path).to_string_lossy()
    //             );
    //         }
    //         Err(e) => panic!("Error: {}", e),
    //     }
    // }

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

        update_unix_shell_rc(&env_vars, temp_config_path.clone())
            .expect("Failed to update temporary shell config file");

        let config_contents = read_to_string(temp_config_path)
            .expect("Failed to read temporary shell config file");

        assert!(
            config_contents.contains("export TEST_VARIABLE_1=\"VALUE1\""),
            "Expected environment variable not found in temporary shell config file"
        );
        assert!(
            config_contents.contains("export TEST_VARIABLE_2=\"VALUE2\""),
            "Expected environment variable not found in temporary shell config file"
        );
    }
}