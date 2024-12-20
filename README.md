<div align="center">
  <img width="150" height="150" src="github_assets/logo.png">
  <h1>BuildIt</h1>
  <p><em>Cross-Platform Build Automation</em></p>
  
  [![Codeac](https://static.codeac.io/badges/2-903404923.svg "Codeac")](https://app.codeac.io/github/neoapps-dev/buildit) [![License](https://img.shields.io/github/license/neoapps-dev/buildit.svg?style=flat-square)](https://github.com/neoapps-dev/buildit/blob/master/LICENSE) ![Rust](https://img.shields.io/badge/Rust-000000?logo=Rust&logoColor=white) [![Linux](https://img.shields.io/badge/GNU/Linux-FCC624?logo=linux&logoColor=black)](#) [![Windows](https://custom-icon-badges.demolab.com/badge/Windows-0078D6?logo=windows11&logoColor=white)](#) [![macOS](https://img.shields.io/badge/macOS-000000?logo=apple&logoColor=F0F0F0)](#)
  [![BSD](https://img.shields.io/badge/BSD-000000?logo=freebsd&logoColor=F0F0F0)](#)
</div>

BuildIt is a simple and universal solution for automating build processes across multiple platforms, including Windows, GNU/Linux, Unix, and macOS. It reads a `BuildFile` that defines functions for each platform, and then executes the corresponding commands in a platform-specific manner.

## Features

- **Cross-Platform Support**: Supports Windows, GNU/Linux, macOS and any Unix-like/based OS that Rust supports.
- **Customizable Build Functions**: Allows users to define platform-specific commands in a `BuildFile`.
- **Configurations**: Uses a simple format to change specific things in the building process.

## Prerequisites

- **Rust**: The Rust programming language is required to build the application.
- **Supported OS**: Windows, GNU/Linux, macOS, or any Unix-like/based OS.

## Installation

Follow these steps to get BuildIt up and running on your system:

1. **Clone the repository**:

   ```bash
   git clone https://github.com/neoapps-dev/buildit.git
   ```

2. **Navigate into the project directory**:

   ```bash
   cd buildit
   ```

3. **Build the project using Cargo**:

   ```bash
   cargo build --release
   ```

4. **Locate the executable** in the `target/release` directory after the build is complete.

5. **Add to PATH**: Add the `target/release/buildit` or `target\release\buildit.exe` if you're on Windows to your PATH.

## Usage

Once built and added to PATH, you can use BuildIt to automate build tasks on different platforms by specifying a function name.

### Command Syntax

```bash
buildit <FunctionName>
```

### Example

To execute a `build` function defined in your `BuildFile`, run:

```bash
buildit build
```

## `BuildFile` Format

The `BuildFile` is where platform-specific functions are defined. It uses a simple format to specify commands to be executed on different platforms.

### Example `BuildFile`

```BuildFile

# An Optional Config Function
config:buildit {
# Turns off using PowerShell on Windows. default: `false`
usePowershellOnWindows: false
# Turns off using PowerShell 7 instead of the default PowerShell 5 on Windows. default: `false`
usePowershell7: false
}

# Windows Script with a Batch Script
build:windows {
    @echo off
    echo "Building project on Windows"
    cargo build --release
}

# GNU/Linux Script with Bash
build:lignux {
    #!/bin/bash
    echo "Building project on GNU/Linux"
    cargo build --release
}

# macOS Script with zsh
build:macos {
    #!/bin/zsh
    echo "Building project on macOS"
    cargo build --release
}

# Unix/BSD Script with sh
build:unix {
    #!/bin/sh
    echo "Hello from Unix!"
}
```

### Format Explanation

- The first part (`build:windows {`) specifies the function name (`build`) and the platform (`windows`).
- The commands inside the curly braces are the steps that will be executed on the specified platform.
- Each platform-specific section can include its own commands, such as `cargo build`, `gcc`, or any other shell command.

## How It Works

1. **Platform Detection**: BuildIt detects the current platform (Windows, GNU/Linux, Unix, or macOS).
2. **Function Lookup**: It looks for a function in the `BuildFile` that matches the specified function name.
3. **Command Execution**: The corresponding commands for the detected platform are extracted and executed in a temporary script (either `.bat` for Windows or `.sh` for Unix-like systems).
4. **Error Handling**: If a function or platform-specific command is missing, BuildIt will output an error message and terminate the process.

## Error Handling

If there are errors during command execution, BuildIt will display:

- An error message with the failed command.
- The specific error status for the command execution.

## License

This project is licensed under the [GNU General Public License v3.0](LICENSE) License.
