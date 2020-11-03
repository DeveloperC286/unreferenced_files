# Unreferenced Files
[![crates.io](https://img.shields.io/crates/v/unreferenced_files)](https://crates.io/crates/unreferenced_files) [![pipeline status](https://gitlab.com/DeveloperC/unreferenced_files/badges/master/pipeline.svg)](https://gitlab.com/DeveloperC/unreferenced_files/commits/master) [![License: AGPL v3](https://img.shields.io/badge/License-AGPLv3-blue.svg)](https://www.gnu.org/licenses/agpl-3.0)


A tool for parsing directories scanning all the files within to find unused/unreferenced files.


## Content
 * [Usage](#usage)
    + [Usage - Example](#usage-example)
    + [Usage - Logging](#usage-logging)
 * [Compiling via Local Repository](#compiling-via-local-repository)
 * [Compiling via Cargo](#compiling-via-cargo)
 * [Issues/Feature Requests](#issuesfeature-requests)


## Usage
`unreferenced_files` is a very simple and fast tool. 
All files inside a directory provided via the argument `--from <from>` are recorded.
The referencing of these files are searched for inside the directory provided via the argument `--search <search>`.

The searching for the referencing of a file is a simple regex of the relative path of the file, inside each file in the searched directory.

e.g.

```
> tree parent

parent/
├── child
│   └── file2.txt
└── file1.txt
```

For the example directory above, if the argument was `--from parent/` then the regex searched for would be `parent/file1.txt` and `parent/child/file2.txt`.

If you changed into the parent directory and used the argument `--from ./` then the regex searched for would be `file1.txt` and `child/file2.txt`.


### Usage - Example
For an example Java project with tests referencing files  inside `src/test/resources/` where the tests are calling the files by name e.g.

```
@Test
public void testImportingFile() {
    ...
    import("file.txt");
    ...
    import("/JSON/file.json");
    ...
}
```

You can find all the unreferenced files inside `src/test/resources/` via

```
cd src/test/resources/
unreferenced_files --from ./ --search ../java/
```


### Usage - Logging
The crates `pretty_env_logger` and `log` are used to provide logging.
The environment variable `RUST_LOG` can be used to set the logging level.
See [https://crates.io/crates/pretty_env_logger](https://crates.io/crates/pretty_env_logger) for more detailed documentation.


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