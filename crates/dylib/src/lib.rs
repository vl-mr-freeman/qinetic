//! [![](https://github.com/vl-mr-freeman/qinetic/blob/master/assets/qinetic_logo2.png)](https://github.com/vl-mr-freeman/qinetic)
//!
//! Qinetic-dylib forces dynamic linking of Qinetic.
//!
//! # Enabling dynamic linking.
//!
//! ## The recommended way.
//!
//! To enable dynamic linking use the `--features qinetic/dynamic` flag, when using the `cargo run` command:
//!
//! ```sh
//! cargo run --features qinetic/dynamic
//! ```
//!
//! ## The unrecommended way.
//!
//! To enable dynamic linking inside of the `Cargo.toml` file add the `dynamic` feature to the qinetic dependency:
//! This is unrecommended because it requires you to remove this feature every time you want to create a
//! release build to avoid having to ship additional files with your game.
//!
//! ```
//! features = ["dynamic"]
//! ```

// Force linking of the main Qinetic's features crate.
#[allow(unused_imports)]
use qinetic_internal;
