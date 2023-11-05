# Building
To build the Windows Registry CLI Utility, simply run `cargo build`

# Running Winreg CLI
The Windows Registry CLI Utility uses the `clap` command line parser. Documentation for commands can be found using the help arguments `-h` or `--help`.

## Prerequisites
The Windows Registry CLI Utility requires Administrator privileges when running the `export` command.

## Executing using cargo
1. Start an elevated terminal session
2. Execute `cargo run help` to see the available commands

## Executing compiled binary
1. Build the binary in either debug or release mode (Debug: `cargo build`, Release: `cargo build -r`)
2. Execute the binary compiled binary (Debug: `.\target\debug\winreg_cli.exe help`, Release: `.\target\release\winreg_cli.exe help`) to see the available commands

# Available Commands

The Windows Registry CLI Utility supports the following top-level commands. The most up to date documentation can be retrieved using the CLI documentation, by passing the help (`-h`) argument.
* Export
* Interrogate **(WIP)**

## Export Quick Start
The `export` command exports the registry hive for root keys to the file system, using `reg save` under the hood (See https://learn.microsoft.com/en-us/windows-server/administration/windows-commands/reg-save).

The valid root keys are **HKLM**, **HKCU**, **HKCR**, **HKU**, and **HKCC**. The export command accepts both the short-hand name (for example, **HKLM**) and the long names (for example, **HKEY_LOCAL_MACHINE**) for the root keys.

Additionally, the export command expects that at least one subkey is specified, for example, **HKLM\\SOFTWARE**.

Multiple keys may be exported in one command by passing multiple key (`-k`) arguments

**Example:**
```
// Export HKEY_LOCAL_MACHINE\\SOFTWARE and HKEY_LOCAL_MACHINE\\SYSTEM to directory .\hive\HKEY_LOCAL_MACHINE-<KEY_NAME>.dat
cargo run export -o hive -k HKLM\\SOFTWARE -k HKLM\\SYSTEM
```

## Interrogate Quick Start (WIP)
The `interrogate` command is used to interrogate the windows registry, finding specific windows registry keys and their associated values. 
