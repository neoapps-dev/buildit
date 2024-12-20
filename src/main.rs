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
    #[cfg(any(target_os = "linux", target_os = "android"))]
    {
        "lignux"
    }
    #[cfg(target_os = "macos")]
    {
        "macos"
    }
    #[cfg(any(target_os = "freebsd", target_os="openbsd", target_os="netbsd"))]
    {
        "unix"
    }
    #[cfg(not(any(windows, target_os = "linux", target_os = "android" ,target_os = "macos", target_os = "freebsd", target_os = "netbsd", target_os = "openbsd")))]
    {
        "unknown"
    }
}


fn execute_command(command: &str, platform: &str, use_powershell: bool, pass_args: bool) {
    let args: Vec<String> = env::args().collect();
    println!("[BuildIt] [INFO] Executing {}:{}", &args[1], platform);
    let full_command: &str =if pass_args {&format!("{} {}", command, args.iter().skip(2).map(|s| s.as_str()).collect::<Vec<&str>>().join(" "))} else {command};
    let executable = full_command.split(" ").nth(0).unwrap();
    let arg1 = if executable == "cmd" {"/c"} else if executable == "powershell" || executable == "pwsh" {"-Command"} else {"-c"};

    let output =  Command::new(executable)
                            .arg(arg1)
                            .arg(&full_command)
                            .output()
                            .expect("[BuildIt] [ERROR] Error executing on Windows");

    if !output.stdout.is_empty() {
        let stdout = String::from_utf8_lossy(&output.stdout);
        print!("{}", stdout);
    }

    if !output.stderr.is_empty() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        eprint!("{}", stderr);
    }

    if !output.status.success() {
        eprintln!("[BuildIt] [ERROR] Failed with status: {}", output.status);
        exit(1);
    }

    if output.status.success() {
        if platform == "windows" && !use_powershell {
            Command::new("powershell")
                .arg("-Command")
                .arg("Remove-Item -Path .\\temp-pwsh.ps1")
                .output()
                .expect("[BuildIt] [ERROR] Error deleting temp file on Windows with Powershell");
        } else if platform == "windows" && use_powershell {
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

fn parse_build_file(filename: &str) -> (HashMap<String, HashMap<String, String>>, bool, bool, bool) {
    let mut functions: HashMap<String, HashMap<String, String>> = HashMap::new();
    let file = File::open(filename).expect("[BuildIt] [ERROR] Could not open BuildFile");
    let reader = io::BufReader::new(file);
    let lines: Vec<String> = reader.lines().map(|line| line.expect("[BuildIt] [ERROR] Error reading line from BuildFile")).collect();

    let mut multiline_command = String::new();
    let mut current_function = String::new();
    let mut current_platform = String::new();
    let mut use_powershell_on_windows = false;
    let mut use_powershell7 = false;
    let mut pass_args = true;

    let mut in_config_block = false;
    for line in &lines {
        let line = line.trim().to_string();

        if line.is_empty() || line.starts_with('#') || line.starts_with(':') {
            continue;
        }

        if line.starts_with("config:buildit {") {
            in_config_block = true;
            continue;
        }

        if in_config_block {
            let configline: &str = line.trim();
            if configline.is_empty() || configline.starts_with('#') {
                continue;
            }
            if configline == "}" {
                in_config_block = false;
                continue;
            }
            if configline == "usePowershellOnWindows: true" {
                use_powershell_on_windows = true;
            }
            if configline == "usePowershellOnWindows: false" {
                use_powershell_on_windows = false;
            }
            if configline == "usePowershell7: true" {
                use_powershell7 = true;
            }
            if configline == "usePowershell7: false" {
                use_powershell7 = false;
            }
            if configline == "passArgs: true" {
                pass_args = true;
            }
            if configline == "passArgs: false" {
                pass_args = false;
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
    (functions, use_powershell_on_windows, use_powershell7, pass_args)
}


fn execute_platform_function(function_name: &str, platform: &str, functions: &HashMap<String, HashMap<String, String>>, use_powershell: bool, use_powershell7: bool, pass_args: bool) {
    if let Some(platform_map) = functions.get(function_name) {
        if let Some(command) = platform_map.get(platform) {
            let filename = if platform == "windows" && use_powershell {
                "temp-pwsh.ps1"
            } else if platform == "windows" {
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

            if platform == "windows" && !use_powershell {
                execute_command(&format!("cmd /c {}", filename), platform, use_powershell, pass_args);
            } else if platform == "windows" && use_powershell && !use_powershell7 {
                execute_command(&format!("powershell -File {}", filename), platform, use_powershell, pass_args);
            } else if platform == "windows" && use_powershell && use_powershell7 {
                execute_command(&format!("pwsh -File {}", filename), platform, use_powershell, pass_args);
            } else {
                execute_command(&format!("sh {}", filename), platform, use_powershell, pass_args);
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
        println!("
██████╗ ██╗   ██╗██╗██╗     ██████╗ ██╗████████╗
██╔══██╗██║   ██║██║██║     ██╔══██╗██║╚══██╔══╝
██████╔╝██║   ██║██║██║     ██║  ██║██║   ██║   
██╔══██╗██║   ██║██║██║     ██║  ██║██║   ██║   
██████╔╝╚██████╔╝██║███████╗██████╔╝██║   ██║   
╚═════╝  ╚═════╝ ╚═╝╚══════╝╚═════╝ ╚═╝   ╚═╝    CLI
                                                
        ");
        println!("Welcome to BuildIt!");
        println!("The Universal Solution for Cross-Platform Build Automation.");
        println!("
        Usage: 
        {} <FunctionName>

        Arguments:
        <FunctionName> ==> The name of the function to execute from the BuildFile.",args[0].replace("\\", "/").replace(".exe", "").split("/").last().unwrap());
        exit(0);
    }

    let function_name = &args[1];
    let current_os = get_os_type();
    let build_file = "BuildFile";

    let (functions, use_powershell_on_windows, use_powershell7, pass_args) = parse_build_file(build_file);
    execute_platform_function(function_name, current_os, &functions, use_powershell_on_windows, use_powershell7, pass_args);
}