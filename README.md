<div align="center">
  <img src="assets/qinetic_logo2.png" alt="Qinetic" />
</div>

[![Crates.io](https://img.shields.io/crates/v/qinetic.svg)](https://crates.io/crates/qinetic)
[![License (MIT)](https://img.shields.io/crates/l/qinetic.svg)](https://github.com/vl-mr-freeman/qinetic/blob/master/LICENSE)
[![docs.rs](https://img.shields.io/badge/docs-website-blue)](https://docs.rs/qinetic/)
[![Dependency status](https://deps.rs/repo/github/vl-mr-freeman/qinetic/status.svg)](https://deps.rs/repo/github/vl-mr-freeman/qinetic)
[![Lines of code](https://tokei.rs/b1/github/vl-mr-freeman/qinetic)](https://github.com/vl-mr-freeman/qinetic)

## About
Qinetic is free, cross-platform, open-source game engine, designed to be fast, simple and modular.

## Docs
* **[Qinetic Rust API Docs](https://docs.rs/qinetic):** Qinetic's Rust API docs, which are automatically generated from the doc comments in this repo.

## Getting started
Recommended to checking out [Qinetic Rust API Docs](https://docs.rs/qinetic) for a full tutorial.

To create a simple application with standard functionality enabled, use:

```rust
use qinetic::prelude::*;

fn main() {
    Application::new()
        .add_system(DefaultSystem)
        .run();
}

```

