//! Network functionality for Qinetic.

#![doc(
    html_logo_url = "https://raw.githubusercontent.com/vl-mr-freeman/qinetic/master/assets/qinetic_icon.svg",
    html_favicon_url = "https://raw.githubusercontent.com/vl-mr-freeman/qinetic/master/assets/qinetic_icon.svg"
)]

pub mod prelude {
    //! Main network functionality.

    #[doc(hidden)]
    pub use crate::{NetworkPlugin, NetworkStage};
}

use qinetic_app::prelude::*;

/// Network functionality for [`App`]
///
/// [`Stage`]s:
/// * [`NetworkStage`]
///
/// # Examples
/// ```
/// # use qinetic_app::prelude::*;
/// # use qinetic_network::prelude::*;
/// #
/// App::builder().with_plugin(NetworkPlugin::default()).build().run();
/// ```
#[derive(Default)]
pub struct NetworkPlugin {}

impl Plugin for NetworkPlugin {
    fn build(&mut self, app_builder: &mut AppBuilder) {
        app_builder.with_stage(NetworkStage::default());
    }
}

/// Network [`Stage`]] for [`App`].
///
/// # Examples
/// ```
/// # use qinetic_app::prelude::*;
/// # use qinetic_networl::prelude::*;
/// #
/// App::builder().with_stage(NetworkStage::default()).build().run();
/// ```
#[derive(Default, Stage)]
pub struct NetworkStage {}
