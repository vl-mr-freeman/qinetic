//! Audio functionality for Qinetic.

#![doc(
    html_logo_url = "https://raw.githubusercontent.com/vl-mr-freeman/qinetic/master/assets/qinetic_icon.svg",
    html_favicon_url = "https://raw.githubusercontent.com/vl-mr-freeman/qinetic/master/assets/qinetic_icon.svg"
)]

pub mod prelude {
    //! Main audio functionality.

    #[doc(hidden)]
    pub use crate::{AudioPlugin, AudioStage, Listener, Sound};
}

use qinetic_app::prelude::*;
use qinetic_ecs::prelude::*;

/// Audio [`Plugin`] for [`App`]
///
/// [`Component`]s:
/// * [`Listener`]
/// * [`Sound`]
///
/// # Examples
/// ```
/// # use qinetic_app::prelude::*;
/// # use qinetic_audio::prelude::*;
/// #
/// App::builder().with_plugin(AudioPlugin::default()).build().run();
/// ```
#[derive(Default)]
pub struct AudioPlugin {}

impl Plugin for AudioPlugin {
    fn build(&mut self, app_builder: &mut AppBuilder) {
        app_builder.with_component(Listener::default());
        app_builder.with_component(Sound::default());
        app_builder.with_stage(AudioStage::default());
    }
}

/// Audio [`Stage`] for [`App`].
///
/// # Examples
/// ```
/// # use qinetic_app::prelude::*;
/// # use qinetic_audio::prelude::*;
/// #
/// App::builder().with_stage(AudioStage::default()).build().run();
/// ```
#[derive(Default, Stage)]
pub struct AudioStage {}

/// Sound [`Component`].
///
/// # Examples
/// ```
/// # use qinetic_app::prelude::*;
/// # use qinetic_audio::prelude::*;
/// #
/// App::builder().with_component(Sound::default()).build().run();
/// ```
#[derive(Default, Component)]
pub struct Sound {}

/// Listener [`Component`].
///
/// # Examples
/// ```
/// # use qinetic_app::prelude::*;
/// # use qinetic_audio::prelude::*;
/// #
/// App::builder().with_component(Listener::default()).build().run();
/// ```
#[derive(Default, Component)]
pub struct Listener {}
