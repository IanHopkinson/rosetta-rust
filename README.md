# Rosetta - Rust

## About

This GitHub repo accompanies a [blog post]() which describes the tooling ecosystem for Rust. This repo provides the technical details in a concise form to accompany the more discussive blog post.

## Rust installation

Rust is installed using the `rustup` tool which can be found [here](https://www.rust-lang.org/tools/install). `rustup update` installs the Rust compiler (rustc), the package manager (cargo), clippy (the linter), rust-docs, and rustfmt.


## Rust Build System and Package Management

Rust package management and building is done using the `cargo` tool, actually `cargo` does pretty much everything.

A Rust package is configured using the `cargo.toml` file which has a dependencies section to indicate which packages the current project requires. During the build process the `cargo.lock` file is generated automatically and should be committed to source control. Although binaries and libraries can be created by directly invoking the rustc compiler usually they a compiled using the `cargo build` command. The `cargo.toml` file also includes a Rust "edition", new editions of Rust are released every 3 years and may contain breaking changes. However, a package is pinned to its initial version and cargo contains tools to help with migration between versions.


## Virtual Environments

In Rust the idea of a virtual environment, as used in Python, is meaningless. 

## Project Layout

New Rust projects are created using `cargo new [project-name] --lib` or `cargo new [project-name] --bin`, this creates a skeleton project layout and initialises a git repo. 

The full layout can be seen [here](https://doc.rust-lang.org/cargo/guide/project-layout.html)

The `cargo.toml` file sits in the root of the project. Typically there will be the following directories:

- `src` - to contain project source Rust files
- `tests` - to contain project test Rust files

And once the package is built there will be a `target` directory. Users can choose to create `benchmark` and `examples` directories for benchmarks and examples. 


## Testing

Typically developers will add tests to library code files, as illustrated in the example `lib.rs` provided when a project is initialized with `cargo new --lib`. In addition doctests can be added to module files, and separate integration tests can be added to the `tests` directory in the root of the project.

All three types of tests can be discovered and run using `cargo test`.

## Linting and formatting tools

Formatting and linting are carried out using tools typically installed by default with `rustup`; formating with `rustfmt` and linting with `clippy`. These are invoked with using `cargo fmt` and `cargo clippy` respectively. `rustfmt` is configured using a configuration file `rustfmt.toml` or `.rustfmt.toml` whose contents are described [here](https://rust-lang.github.io/rustfmt/?version=v1.6.0&search=) although it will work without configuration. Similarly `clippy` is configured with files like `clippy.toml` or `.clippy.toml` described [here](https://doc.rust-lang.org/clippy/configuration.html). Both tools work pretty well using default configuration.

## Documentation Generation

## Git Bash

A `Makefile` is included in this repository which has targets for installing the package, running tests, running the linters and 
building the documentation. For Git Bash I needed to install `make` using the chocolatey package manager (`choco install make`).


## Visual Studio Code
As a personal choice, I use Visual Studio Code for software development. 

The configuration used for this project for Code is found in the `.vscode` directory of this repo.