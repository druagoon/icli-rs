<h1>icli-rs</h1>

[![GitHub issues](https://img.shields.io/github/issues/druagoon/icli-rs)](https://github.com/druagoon/icli-rs/issues)
[![GitHub license](https://img.shields.io/github/license/druagoon/icli-rs)](https://github.com/druagoon/icli-rs/blob/master/LICENSE)

`icli` is a personal command-line tool, which includes various commonly used utilities.

-----

**Table of Contents**

- [Installation](#installation)
  - [From binaries](#from-binaries)
  - [From source](#from-source)
- [Configuration](#configuration)
  - [Built-in default config](#built-in-default-config)
- [FAQ](#faq)
  - [How to compile on Windows?](#how-to-compile-on-windows)
- [Changelog](#changelog)
- [License](#license)

## Installation

### From binaries

The [releases page](https://github.com/druagoon/icli-rs/releases) includes precompiled binaries for Linux and Windows.\
Statically-linked binaries are also available: look for archives with `musl` in the file name.

### From source

> MSRV is `1.79.0`
>
> There are some compilation requirements on Windows, see [FAQ](#how-to-compile-on-windows).

- Install `Npcap` required by `pnet` crate (**Windows only**)

```powershell
# Windows PowerShell with administrator permissions.
Invoke-WebRequest -Uri "https://npcap.com/dist/npcap-sdk-1.13.zip" -OutFile "C:/npcap-sdk.zip"
Expand-Archive -LiteralPath C:/npcap-sdk.zip -DestinationPath C:/npcap-sdk
$env:LIB="C:/npcap-sdk/Lib/x64"
```

- Install from github repository

```shell
cargo install --locked --all-features --git https://github.com/druagoon/icli-rs.git
```

## Configuration

`icli` allows local configuration for a particular package as well as global configuration.
It looks for configuration files in the current directory and user configuration directory (in precedence order).

- `$PWD/.icli/config.toml`
- `~/.config/icli/config.toml`:
  - Windows: `%USERPROFILE%\.config\icli\config.toml`
  - Unix: `$HOME/.config/icli/config.toml`

If a key is specified in multiple config files, the values will get merged together.\
Numbers, strings, booleans and arrays will use the value in the current directory configuration first,
followed by the user configuration directory.\
Tables will be joined together with higher precedence items being override those with lower precedence.

> Note:
***In fact, the built-in default configuration is the lowest priority.***

### Built-in default config

- Global
  - [default.toml](./templates/config/default.toml)
- OS
  - [linux.toml](./templates/config/linux.toml)
  - [macos.toml](./templates/config/macos.toml)
  - [windows.toml](./templates/config/windows.toml)

## FAQ

### How to compile on Windows?

`icli` depends on the `pnet` crate.
On Windows systems,`pent` crate has some compilation requirements,
please refer to the [libpnet](https://github.com/libpnet/libpnet?tab=readme-ov-file#windows).

## Changelog

See [CHANGELOG.md](./CHANGELOG.md).

## License

`icli` is distributed under the terms of the [MIT](https://spdx.org/licenses/MIT.html) license.
