//! Core functionality for Qinetic.

#![doc(
    html_logo_url = "https://raw.githubusercontent.com/vl-mr-freeman/qinetic/master/assets/qinetic_icon.svg",
    html_favicon_url = "https://raw.githubusercontent.com/vl-mr-freeman/qinetic/master/assets/qinetic_icon.svg"
)]

pub mod prelude {
    //! Provides main core functionality.
    #[doc(hidden)]
    pub use crate::{color::*, CorePlugin};
}

pub mod color {
    //! Provides color facilitate creating.
    pub use crate::{color3::*, color4::*};

    use std::str::Bytes;

    pub(crate) fn parse_double_hex_value(bytes: &mut Bytes) -> Option<u8> {
        const HEX_RADIX: u32 = 16;
        let buffer = [
            match bytes.next() {
                Some(b) => b,
                None => return None,
            },
            match bytes.next() {
                Some(b) => b,
                None => return None,
            },
        ];
        let s = core::str::from_utf8(&buffer);
        let s = match s {
            Ok(s) => s,
            Err(_) => return None,
        };
        u8::from_str_radix(s, HEX_RADIX).ok()
    }
}

mod color3;
mod color4;

use qinetic_app::prelude::*;

/// Adds core functionality to [`App`].
#[derive(Default)]
pub struct CorePlugin;

impl Plugin for CorePlugin {
    fn build(&self, app_builder: &mut AppBuilder) {}
}
