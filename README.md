# Rosetta - Rust

## About

This GitHub repo accompanies a [blog post](https://ianhopkinson.org.uk/2025/01/rosetta-stone-rust/) which describes the tooling ecosystem for Rust. This repo provides the technical details in a concise form to accompany the more discussive blog post. The application created is a simple word count tool, invoked with:

```
cargo run -- tests/fixtures/word_count_test_file.txt
```

where `tests/fixtures/word_count_test_file.txt` is the path to the file on which to count words. The wordcount binary can be installed as a commandline tool with:

```
cargo install --path .
```

And then invoked with:

```
wordcount_cli tests/fixtures/word_count_test_file.txt
```

## Rust installation

Rust is installed using the `rustup` tool which can be found [here](https://www.rust-lang.org/tools/install). `rustup update` installs the Rust compiler (`rustc`), the package manager (`cargo`), `clippy` (the linter), `rust-docs`, and `rustfmt`.


## Rust Build System and Package Management

Rust package management and building is done using the `cargo` tool, actually `cargo` does pretty much everything.

A Rust package is configured using the `cargo.toml` file which has a dependencies section to indicate which packages the current project requires. During the build process the `cargo.lock` file is generated automatically and should be committed to source control. Although binaries and libraries can be created by directly invoking the `rustc` compiler usually they a compiled using the `cargo build` command. The `cargo.toml` file also includes a Rust "edition", new editions of Rust are released every 3 years and may contain breaking changes. However, a package is pinned to its initial version and cargo contains tools to help with migration between versions.

## Project Layout

New Rust projects are created using `cargo new [project-name] --lib` or `cargo new [project-name] --bin`, this creates a skeleton project layout and initialises a Git repo. This repo contains both a binary and a library Rust file so the `cargo.toml` is modified to accomodate this.

The full layout can be seen [here](https://doc.rust-lang.org/cargo/guide/project-layout.html)

The `cargo.toml` file sits in the root of the project. Typically there will be the following directories:

- `src` - to contain project source Rust files
- `tests` - to contain project test Rust files

And once the package is built there will be a `target` directory. Users can choose to create `benchmark` and `examples` directories for benchmarks and examples. Documentation built using rustdoc ends up in a subdirectory of the `target` directory.


## Testing

This repo contains unit tests, doctests and an integration test which can be run with:

```
cargo test
```

## Linting and formatting tools

Formatting and linting are carried out using tools typically installed by default with `rustup`; formating with `rustfmt` and linting with `clippy`. These are invoked with using `cargo fmt` and `cargo clippy` respectively. `rustfmt` is configured using a configuration file `rustfmt.toml` or `.rustfmt.toml` whose contents are described [here](https://rust-lang.github.io/rustfmt/?version=v1.6.0&search=) although it will work without configuration. Similarly `clippy` is configured with files like `clippy.toml` or `.clippy.toml` described [here](https://doc.rust-lang.org/clippy/configuration.html). Both tools work pretty well using default configuration.

Since I was using Visual Code this `clippy` and `rustfmt` were run automatically in the background.

## Documentation Generation

HTML format documentation can be generated using the built-in `rustdoc` tool which can be invoked using `cargo`:

`cargo doc --open`

This compiles documentation from function comments marked `///` or `!//` for module level documentation.

## Git Bash

Although I develop on Windows I use the Git Bash prompt. For other languages it is sometimes useful to have a `make` file to invoke tools but for Rust `cargo` is sufficiently functional for it not to be required. 

## Visual Studio Code
As a personal choice, I use Visual Studio Code for software development. There is a [rust-analyzer](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust-analyzer) which supports development in Rust. The settings I used for this project can be found in `.vscode/settings.json` the only changes are to explicitly use `clippy` for linting and show tests in the test explorer.

