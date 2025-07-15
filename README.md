# Unreferenced Files
[![crates.io](https://img.shields.io/crates/v/unreferenced_files)](https://crates.io/crates/unreferenced_files)
[![Conventional Commits](https://img.shields.io/badge/Conventional%20Commits-1.0.0-yellow.svg)](https://conventionalcommits.org)
[![License](https://img.shields.io/badge/License-AGPLv3-blue.svg)](https://www.gnu.org/licenses/agpl-3.0)

A utility for finding unused and unreferenced files.

- [Usage](#usage)
  - [Additional Arguments](#additional-arguments)
  - [Example](#example)
- [Examples](#examples)
  - [GitHub Actions](#github-actions)
  - [GitLab CI](#gitlab-ci)
- [Installation](#installation)
  - [Binary](#binary)
  - [Cargo](#cargo)
  - [Docker](#docker)

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
│   └── file2.txt
└── file1.txt
```

For the example directory above, if the argument was `--search-for parent/` then for the file `parent/file1.txt` the relative path of `parent/file1.txt`, the file name `file1.txt`, and the file stem `file1` would be searched for.
For the file `parent/child/file2.txt` the relative path of `parent/child/file2.txt`, the file name `file2.txt` and file stem `file2` would be searched for.

### Additional Arguments

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

### Example
For an example Java project with tests referencing files inside `src/test/resources/` where the tests are calling the files by name e.g.

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

## Examples
### GitHub Actions
<!-- x-release-please-start-version -->
```yaml
name: CI

on: pull_request

jobs:
  check-unreferenced-files:
    name: Check Unreferenced Files
    runs-on: ubuntu-latest
    steps:
      - name: Checkout code.
        uses: actions/checkout@v4
      - name: Install Unreferenced Files.
        run: version="v2.1.1" && wget -O - "https://github.com/DeveloperC286/unreferenced_files/releases/download/${version}/x86_64-unknown-linux-musl.tar.gz" | tar xz --directory "/usr/bin/"
      - name: Check for unreferenced files
        run: unreferenced_files --search-for src/test/resources/ --search src/test/java/ --assert-no-unreferenced-files
```
<!-- x-release-please-end -->

### GitLab CI
<!-- x-release-please-start-version -->
```yaml
check-unreferenced-files:
  stage: check-unreferenced-files
  image: rust
  before_script:
    - version="v2.1.1" && wget -O - "https://github.com/DeveloperC286/unreferenced_files/releases/download/${version}/x86_64-unknown-linux-musl.tar.gz" | tar xz --directory "/usr/bin/"
  script:
    - unreferenced_files --search-for src/test/resources/ --search src/test/java/ --assert-no-unreferenced-files
  rules:
    - if: $CI_MERGE_REQUEST_ID

```
<!-- x-release-please-end -->

## Installation

### Binary
<!-- x-release-please-start-version -->
```sh
version="v2.1.1" && wget -O - "https://github.com/DeveloperC286/unreferenced_files/releases/download/${version}/x86_64-unknown-linux-musl.tar.gz" | tar xz --directory "/usr/bin/"
```
<!-- x-release-please-end -->

### Cargo
Cargo is the Rust package manager, the `install` sub-command pulls from [crates.io](https://crates.io/crates/unreferenced_files) and then compiles the binary locally, placing the compiled binary at `${HOME}/.cargo/bin/unreferenced_files`.

```sh
cargo install unreferenced_files
```

By default it installs the latest version at the time of execution.
You can specify a specific version to install using the `--version` argument.
For certain environments such as CICD etc you may want to pin the version.

<!-- x-release-please-start-version -->
e.g.

```sh
cargo install unreferenced_files --version 2.1.1
```
<!-- x-release-please-end -->

Rather than pinning to a specific version you can specify the major or minor version.

<!-- x-release-please-start-version -->
e.g.

```sh
cargo install unreferenced_files --version ^2
```
<!-- x-release-please-end -->

Will download the latest `2.*` release whether that is `2.0.0` or `2.7.0`.

### Docker
You can use the Docker image published to [ghcr.io/developerc286/unreferenced_files](https://github.com/DeveloperC286/unreferenced_files/pkgs/container/unreferenced_files).

## Issues/Feature Requests
Report issues or request features on our [GitHub Issues](https://github.com/DeveloperC286/unreferenced_files/issues).
