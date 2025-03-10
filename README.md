<!-- markdownlint-disable MD033 MD036 -->
<h1>icli-rs</h1>

![GitHub license](https://img.shields.io/github/license/druagoon/icli-rs)
[![GitHub Release](https://img.shields.io/github/v/release/druagoon/icli-rs)](https://github.com/druagoon/icli-rs/releases)
[![GitHub issues](https://img.shields.io/github/issues/druagoon/icli-rs)](https://github.com/druagoon/icli-rs/issues)

A personal command-line tool that includes various commonly used utilities.

-----

**Table of Contents**

- [Installation](#installation)
  - [From Homebrew](#from-homebrew)
  - [From binaries](#from-binaries)
  - [From source](#from-source)
    - [Windows Prerequisites](#windows-prerequisites)
- [Configuration](#configuration)
  - [Configuration Merging](#configuration-merging)
  - [Built-in Default Configs](#built-in-default-configs)
- [Development](#development)
  - [Windows Compilation](#windows-compilation)
- [Changelog](#changelog)
- [License](#license)


## Installation

### From Homebrew

```shell
brew install druagoon/brew/icli
```

### From binaries

Download precompiled binaries for Linux/Windows/MacOS from the [releases page](https://github.com/druagoon/icli-rs/releases).

For Linux users, statically-linked binaries (with `musl`) are also available.

### From source

> Minimum supported Rust version (MSRV): 1.79.0

#### Windows Prerequisites

Install Npcap SDK (required by `pnet` crate):

```powershell
# Run in PowerShell with administrator permissions
Invoke-WebRequest -Uri "https://npcap.com/dist/npcap-sdk-1.13.zip" -OutFile "C:/npcap-sdk.zip"
Expand-Archive -LiteralPath C:/npcap-sdk.zip -DestinationPath C:/npcap-sdk
$env:LIB="C:/npcap-sdk/Lib/x64"
```

Install from repository:

```shell
cargo install --locked --all-features --git https://github.com/druagoon/icli-rs.git
```

## Configuration

Configuration files are loaded in the following order (highest to lowest priority):

1. Local config: `$PWD/.config/icli/config.toml`
2. User config:
   - Windows: `%USERPROFILE%\.config\icli\config.toml`
   - Unix: `$HOME/.config/icli/config.toml`
3. Built-in defaults (lowest priority)

### Configuration Merging

- Simple values (numbers, strings, booleans, arrays): Higher priority overrides lower
- Tables: Merged with higher priority values overriding lower priority ones

### Built-in Default Configs

- Global: [default.toml](./templates/config/default.toml)
- OS-specific:
  - [linux.toml](./templates/config/os/linux.toml)
  - [macos.toml](./templates/config/os/macos.toml)
  - [windows.toml](./templates/config/os/windows.toml)

## Development

### Windows Compilation

This project depends on the `pnet` crate. For Windows compilation requirements, please refer to [libpnet documentation](https://github.com/libpnet/libpnet?tab=readme-ov-file#windows).

## Changelog

See [CHANGELOG.md](./CHANGELOG.md).

## License

`icli` is licensed under the [MIT License](https://spdx.org/licenses/MIT.html).
