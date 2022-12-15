//! Core functionality for Qinetic.

#![doc(
    html_logo_url = "https://raw.githubusercontent.com/vl-mr-freeman/qinetic/master/assets/qinetic_icon.svg",
    html_favicon_url = "https://raw.githubusercontent.com/vl-mr-freeman/qinetic/master/assets/qinetic_icon.svg"
)]

pub mod prelude {
    //! Main core functionality.

    #[doc(hidden)]
    pub use crate::{color::*, CorePlugin, CoreStage, Script, Tag};
}

pub mod color {
    //! Color functionality.

    pub use crate::{color3::*, color4::*};

    use std::str::Bytes;

    /// Return `u8` parsed from [`Bytes`].
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
use qinetic_ecs::prelude::*;

/// Core [`Plugin`] for [`App`].
///
/// [`Component`]s:
/// * [`Tag`]
/// * [`Script`]
///
/// [`Stage`]s:
/// * [`CoreStage`]
///
/// # Examples
/// ```
/// # use qinetic_app::prelude::*;
/// # use qinetic_core::prelude::*;
/// #
/// App::builder().with_plugin(CorePlugin::default()).build().run();
/// ```
#[derive(Default)]
pub struct CorePlugin {}

impl Plugin for CorePlugin {
    fn build(&mut self, app_builder: &mut AppBuilder) {
        app_builder.with_component(Tag::default());
        app_builder.with_component(Script::default());
        app_builder.with_stage(CoreStage::PreInit);
        app_builder.with_stage(CoreStage::Init);
        app_builder.with_stage(CoreStage::PostInit);
        app_builder.with_stage(CoreStage::PreUpdate);
        app_builder.with_stage(CoreStage::Update);
        app_builder.with_stage(CoreStage::PostUpdate);
        app_builder.with_stage(CoreStage::PreTerm);
        app_builder.with_stage(CoreStage::Term);
        app_builder.with_stage(CoreStage::PostTerm);
    }
}

/// Core [`Stage`].
/// # Examples
/// ```
/// # use qinetic_app::prelude::*;
/// # use qinetic_ecs::prelude::*;
/// # use qinetic_core::prelude::*;
/// #
/// App::builder().with_stage(CoreStage::PreInit).build().run();
/// ```
#[derive(Stage)]
pub enum CoreStage {
    /// The [`Stage`] that runs once before [`CoreStage::Init`].
    PreInit,

    /// The [`Stage`] that runs once on running [`App`].
    Init,

    /// The [`Stage`] that runs once after [`CoreStage::Init`].
    PostInit,

    /// The [`Stage`] that runs before [`CoreStage::Update`].
    PreUpdate,

    /// The [`Stage`] that runs on updating [`App`].
    Update,

    /// The [`Stage`] that runs after [`CoreStage::Update`].
    PostUpdate,

    /// The [`Stage`] that runs once before [`CoreStage::Term`].
    PreTerm,

    /// The [`Stage`] that runs once on closing [`App`].
    Term,

    /// The [`Stage`] that runs once after [`CoreStage::Term`].
    PostTerm,
}

/// Tag [`Component`].
///
/// # Examples
/// ```
/// # use qinetic_app::prelude::*;
/// # use qinetic_core::prelude::*;
/// #
/// App::builder().with_component(Tag::default()).build().run();
/// ```
#[derive(Default, Component)]
pub struct Tag {
    pub tag: String,
}

/// Script [`Component`].
///
/// # Examples
/// ```
/// # use qinetic_app::prelude::*;
/// # use qinetic_core::prelude::*;
/// #
/// App::builder().with_component(Script::default()).build().run();
/// ```
#[derive(Default, Component)]
pub struct Script {
    pub name: String,
}
