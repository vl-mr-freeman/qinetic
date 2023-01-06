//! User-interface [`Resource`]s functionality.

use qinetic_ecs::prelude::*;
use qinetic_utils::prelude::*;

use crate::{font::Font, theme::Theme};

/// User-interface [`Resource`].
///
/// # Examples
/// ```
/// # use qinetic_app::prelude::*;
/// # use qinetic_ui::prelude::*;
/// #
/// App::builder()
///     .with_resource(UiResource::default())
///     .build()
///     .unwrap()
///     .run();
/// ```
#[derive(SmartDefault, Clone, Debug, Getters, Builder, Resource)]
#[getset(get = "pub")]
#[builder(
    crate = "crate::resources",
    setter(prefix = "with"),
    default,
    derive(Debug, PartialEq)
)]
pub struct UiResource {
    /// The user-interface client opacity.
    #[default = 1.0]
    opacity: f32,

    /// The user-interface client theme.
    theme: Theme,

    /// The user-interface client font.
    font: Font,

    /// The user-interface client window padding.
    #[default((10.0, 10.0))]
    window_padding: (f32, f32),

    /// The user-interface client window rounding.
    #[default = 5.0]
    window_rounding: f32,

    /// The user-interface client window border size.
    #[default = 1.0]
    window_border_size: f32,

    /// The user-interface client window's child padding.
    #[default((15.0, 15.0))]
    child_window_padding: (f32, f32),

    /// The user-interface client window's child rounding.
    #[default = 5.0]
    child_window_rounding: f32,

    /// The user-interface client window's child border size.
    #[default = 1.0]
    child_window_border_size: f32,

    /// The user-interface client frame padding.
    #[default((5.0, 2.5))]
    frame_padding: (f32, f32),

    /// The user-interface client frame rounding.
    #[default = 2.5]
    frame_rounding: f32,

    /// The user-interface client frame border size.
    #[default = 0.0]
    frame_border_size: f32,

    /// The user-interface client inner window items spacing.
    #[default((5.0, 5.0))]
    item_spacing: (f32, f32),

    /// The user-interface client inner window items inner spacing.
    #[default((5.0, 5.0))]
    item_inner_spacing: (f32, f32),

    /// The user-interface client scrollbar thickness.
    #[default = 12.5]
    scrollbar_thickness: f32,

    /// The user-interface client scrollbar rounding.
    #[default = 5.0]
    scrollbar_rounding: f32,

    /// The user-interface client grab size.
    #[default = 10.0]
    grab_size: f32,

    /// The user-interface client grab rounding.
    #[default = 2.5]
    grab_rounding: f32,
}
