# Rust

To start, you'll need to install the Rust toolchain, [which can be downloaded from rustup.rs](https://rustup.rs/)

After doing this, open a terminal in this folder and run the following:

```
cargo build
cargo run 
```

This will run the build using the provided Cargo.toml file and then run the application.

### VSCode
VS Code users may wish to install the rust-analyzer extension in the marketplace for core language support.

### Initial Setup

During initial setup, it was necessary to create a new project and add dependencies for once_cell (lazy evalulation) and rand (PRG logic) like so:

```
cargo new rust
cargo add once_cell
cargo add rand
```