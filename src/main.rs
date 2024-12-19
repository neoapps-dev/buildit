use std::env;
use std::fs::{File, OpenOptions};
use std::io::{self, BufRead, Write};
use std::process::{Command, exit};
use std::collections::HashMap;

fn get_os_type() -> &'static str {
    #[cfg(windows)]
    {
        "windows"
    }
    #[cfg(target_os = "linux")]
    {
        "linux"
    }
    #[cfg(target_os = "macos")]
    {
        "macos"
    }
    #[cfg(target_os = "unix")]
    {
        "unix"
    }
    #[cfg(not(any(windows, target_os = "linux", target_os = "macos", target_os = "unix")))]
    {
        "unknown"
    }
}

fn execute_command(command: &str, platform: &str, use_powershell: bool) {
    let args: Vec<String> = env::args().collect();
    println!("[BuildIt] [INFO] Executing {}:{}", platform, &args[1]);

    let output = if platform == "windows" && use_powershell {
        Command::new("powershell")
            .arg("-Command")
            .arg(command)
            .output()
            .expect("[BuildIt] [ERROR] Error executing command on Windows with PowerShell")
    } else if platform == "windows" {
        Command::new("cmd")
            .arg("/c")
            .arg(command)
            .output()
            .expect("[BuildIt] [ERROR] Error executing command on Windows")
    } else {
        Command::new("sh")
            .arg("-c")
            .arg(command)
            .output()
            .expect("[BuildIt] [ERROR] Error executing command on Unix-like platform")
    };

    if !output.stdout.is_empty() {
        let stdout = String::from_utf8_lossy(&output.stdout);
        print!("{}", stdout);
    }

    if !output.stderr.is_empty() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        eprint!("{}", stderr);
    }

    if !output.status.success() {
        eprintln!("[BuildIt] [ERROR] Command failed with status: {}", output.status);
        exit(1);
    }

    if output.status.success() {
        if platform == "windows" {
            Command::new("cmd")
                .arg("/c")
                .arg("del temp.bat")
                .output()
                .expect("[BuildIt] [ERROR] Error deleting temp file on Windows");
        } else {
            Command::new("sh")
                .arg("-c")
                .arg("rm temp.sh")
                .output()
                .expect("[BuildIt] [ERROR] Error deleting temp file on Unix-like platform");
        }
    }
}

fn parse_build_file(filename: &str) -> (HashMap<String, HashMap<String, String>>, bool) {
    let mut functions: HashMap<String, HashMap<String, String>> = HashMap::new();
    let file = File::open(filename).expect("[BuildIt] [ERROR] Could not open BuildFile");
    let mut multiline_command = String::new();
    let mut current_function = String::new();
    let mut current_platform = String::new();
    let mut use_powershell_on_windows = false;

    for line in io::BufReader::new(file).lines() {
        let line = line.expect("[BuildIt] [ERROR] Error reading line from BuildFile").trim().to_string();

        if line.is_empty() || line.starts_with('#') || line.starts_with(':') {
            continue;
        }

        if line.starts_with("config:buildit {") {
            while let Some(config_line) = io::BufReader::new(file).lines().next() {
                let config_line = config_line.expect("[BuildIt] [ERROR] Error reading line from BuildFile").trim().to_string();
                if config_line.is_empty() || config_line.starts_with('#') {
                    continue;
                }
                if config_line == "}" {
                    break;
                }
                if config_line == "usePowershellOnWindows: true" {
                    use_powershell_on_windows = true;
                }
                if config_line == "usePowershellOnWindows: false" {
                    use_powershell_on_windows = false;
                }
            }
            continue;
        }

        if line.ends_with("{") {
            if !current_function.is_empty() {
                continue;
            }
            let mut parts = line.splitn(3, ':');
            current_function = parts.next().unwrap_or("").trim().to_string();
            current_platform = parts.next().unwrap_or("").trim().to_string();
            continue;
        }

        if line.ends_with("}") {
            let command = line.trim_end_matches('}').trim().to_string();
            multiline_command.push_str(&command);
            multiline_command.push_str("\n");

            if !current_function.is_empty() && !current_platform.is_empty() {
                let function_map = functions.entry(current_function.clone()).or_insert_with(HashMap::new);
                function_map.insert(current_platform.clone().replace(" {", ""), multiline_command.clone());
                multiline_command.clear();
                current_function.clear();
                current_platform.clear();
            }
            continue;
        }

        multiline_command.push_str(&line);
        multiline_command.push_str("\n");
    }
    (functions, use_powershell_on_windows)
}

fn execute_platform_function(function_name: &str, platform: &str, functions: &HashMap<String, HashMap<String, String>>, use_powershell: bool) {
    if let Some(platform_map) = functions.get(function_name) {
        if let Some(command) = platform_map.get(platform) {
            let filename = if platform == "windows" {
                "temp.bat"
            } else {
                "temp.sh"
            };

            let mut file = OpenOptions::new()
                .create(true)
                .write(true)
                .truncate(true)
                .open(filename)
                .expect("[BuildIt] [ERROR] Error creating temporary script file");

            file.write_all(command.as_bytes())
                .expect("[BuildIt] [ERROR] Error writing to temporary script file");

            if platform == "windows" {
                execute_command(&format!("cmd /c {}", filename), platform, use_powershell);
            } else {
                execute_command(&format!("sh {}", filename), platform, use_powershell);
            }
        } else {
            println!("{:#?}", platform_map);
            eprintln!("[BuildIt] [ERROR] No command found for platform: {}", platform);
            exit(1);
        }
    } else {
        eprintln!("[BuildIt] [ERROR] No function found with name: {}", function_name);
        exit(1);
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Welcome to BuildIt!");
        println!("The Universal Solution for Cross-Platform Build Automation.");
        println!("Usage: buildit <FunctionName> (e.g. build, run, etc..)");
        exit(0);
    }

    let function_name = &args[1];
    let current_os = get_os_type();
    let build_file = "BuildFile";

    let (functions, use_powershell_on_windows) = parse_build_file(build_file);
    execute_platform_function(function_name, current_os, &functions, use_powershell_on_windows);
}
