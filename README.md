# Unreferenced Files
[![crates.io](https://img.shields.io/crates/v/unreferenced_files)](https://crates.io/crates/unreferenced_files) [![pipeline status](https://gitlab.com/DeveloperC/unreferenced_files/badges/master/pipeline.svg)](https://gitlab.com/DeveloperC/unreferenced_files/commits/master) [![License: AGPL v3](https://img.shields.io/badge/License-AGPLv3-blue.svg)](https://www.gnu.org/licenses/agpl-3.0)


A tool for parsing directories scanning all the files within to find unused/unreferenced files.


## Content
 * [Usage](#usage)
 * [Compiling via Local Repository](#compiling-via-local-repository)
 * [Compiling via Cargo](#compiling-via-cargo)
 * [Issues/Feature Requests](#issuesfeature-requests)


## Usage


## Compiling via Local Repository
Checkout the code repository locally, change into the repository's directory and then build via cargo.
Using the `--release` flag produces an optimised binary but takes longer to compile.

```
git clone git@gitlab.com:DeveloperC/unreferenced_files.git
cd unreferenced_files/
cargo build --release
```

The compiled binary is present in `target/release/unreferenced_files`.


## Compiling via Cargo
Cargo is the Rust package manager, using the `install` sub-command it pulls the crate from `crates.io` and then compiles the binary locally.
`cargo install` places the produced binary at `$HOME/.cargo/bin/unreferenced_files`.

```
cargo install unreferenced_files
```


## Issues/Feature Requests
To report an issue or request a new feature use [https://gitlab.com/DeveloperC/unreferenced_files/-/issues](https://gitlab.com/DeveloperC/unreferenced_files/-/issues).