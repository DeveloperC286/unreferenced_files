# Unreferenced Files
[![crates.io](https://img.shields.io/crates/v/unreferenced_files)](https://crates.io/crates/unreferenced_files)
[![Conventional Commits](https://img.shields.io/badge/Conventional%20Commits-1.0.0-yellow.svg)](https://conventionalcommits.org)
[![License](https://img.shields.io/badge/License-AGPLv3-blue.svg)](https://www.gnu.org/licenses/agpl-3.0)


A utility for finding unused and unreferenced files.


## Content
 * [Usage](#usage)
    + [Usage - Additional Arguments](#usage-additional-arguments)
    + [Usage - Example](#usage-example)
    + [Usage - Logging](#usage-logging)
 * [Compiling via Local Repository](#compiling-via-local-repository)
 * [Compiling via Cargo](#compiling-via-cargo)
 * [End-to-End Testing](#end-to-end-testing)
 * [Issues/Feature Requests](#issuesfeature-requests)


## Usage
`unreferenced_files` is a very simple and fast tool.
All files inside any of the directories or files provided via the arguments `--search-for <search-for>` are recorded.
The referencing of these files are searched for inside the directories or files provided by the arguments `--search <search>`.

By default, the referencing of a file is search for via looking for the relative path of the file, the file name, and the file stem inside each file.
If the search for and search files overlap then a file will not search itself to finds self references.

e.g.

```sh
tree parent

parent/
├── child
│   └── file2.txt
└── file1.txt
```

For the example directory above, if the argument was `--search-for parent/` then for the file `parent/file1.txt` the relative path of `parent/file1.txt`, the file name `file1.txt`, and the file stem `file1` would be searched for.
For the file `parent/child/file2.txt` the relative path of `parent/child/file2.txt`, the file name `file2.txt` and file stem `file2` would be searched for.


### Usage - Additional Arguments

Additional command line flags can be passed to alter what is searched for to determine if a file is referenced.

| Flag                      | |
|---------------------------|-|
| --only-search-for | Only search for files that match any of these regexes, mutually exclusive with ignore search for. |
| --ignore-search-for | Ignore and do not search for any files that match any of these regexes, mutually exclusive with only search for. |
| --only-search | Only search files that match any of these regexes, mutual exclusive with ignore search. |
| --ignore-search | Ignore and do not search any files that match any of these regexes, mutually exclusive with only search. |
| --only-file-name | Only search for unreferenced files via their file name. |
| --only-file-stem | Only search for unreferenced files via their file name without the extension. |
| --only-relative-path | Only search for unreferenced files via their relative path. |
| --print-full-path | Output the full path of each unreferenced file, instead of the relative path. |
| --assert-no-unreferenced-files | Return a nonzero exit code if there are any unreferenced files. |


### Usage - Example
For an example Java project with tests referencing files  inside `src/test/resources/` where the tests are calling the files by name e.g.

```java
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

```sh
cd src/test/resources/
unreferenced_files --search-for ./ --search ../java/
```


### Usage - Logging
The crates `pretty_env_logger` and `log` are used to provide logging.
The environment variable `RUST_LOG` can be used to set the logging level.
See [https://crates.io/crates/pretty_env_logger](https://crates.io/crates/pretty_env_logger) for more detailed documentation.


## Compiling via Local Repository
Checkout the code repository locally, change into the repository's directory and then build via cargo.
Using the `--release` flag produces an optimised binary but takes longer to compile.

```sh
git clone https://github.com/DeveloperC286/unreferenced_files.git
cd unreferenced_files/
cargo build --release
```

The compiled binary is present in `target/release/unreferenced_files`.


## Compiling via Cargo
Cargo is the Rust package manager, the `install` sub-command pulls from [crates.io](https://crates.io/crates/clean_git_history) and then compiles the binary locally, placing the compiled binary at `${HOME}/.cargo/bin/clean_git_history`.

```sh
cargo install unreferenced_files
```

By default it installs the latest version at the time of execution.
You can specify a specific version to install using the `--version` argument.
For certain environments such as CICD etc you may want to pin the version.

e.g.

```sh
cargo install unreferenced_files --version 2.0.0
```

Rather than pinning to a specific version you can specify the major or minor version.

e.g.

```sh
cargo install unreferenced_files --version ^2
```

Will download the latest `2.*` release whether that is `2.0.0` or `2.7.0`.


## End-to-End Testing
To ensure correctness as there are a variety of out of process dependencies the project has an End-to-End behaviour driven test suite using the behave framework (https://github.com/behave/behave).

To run the test suite you need to
 - Compile the Unreferenced Files binary.
 - Install Python3.
 - Install Behave.
 - Execute Behave.

__Note - You can't use --release as the test suite uses `target/debug/unreferenced_files`.__

```sh
cargo build
cd unreferenced_files/end-to-end-tests/
virtualenv -p python3 .venv
source .venv/bin/activate
pip3 install -r requirements.txt
behave
```


## Issues/Feature Requests
To report an issue or request a new feature use [https://github.com/DeveloperC286/unreferenced_files/issues](https://github.com/DeveloperC286/unreferenced_files/issues).
