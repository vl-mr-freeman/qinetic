//! User-interface functionality for Qinetic.

#![doc(
    html_logo_url = "https://raw.githubusercontent.com/vl-mr-freeman/qinetic/master/assets/qinetic_icon.svg",
    html_favicon_url = "https://raw.githubusercontent.com/vl-mr-freeman/qinetic/master/assets/qinetic_icon.svg"
)]

pub mod prelude {
    //! Main user-interface functionality.

    #[doc(hidden)]
    pub use crate::{font::*, theme::*, UiPlugin, UiResource, UiStage};
}

pub mod font;
pub mod theme;

use crate::font::*;
use crate::theme::*;
use qinetic_app::prelude::*;
use qinetic_ecs::prelude::*;
use qinetic_math::vector::Vec2;

/// User-interface functionality for [`App`]
///
/// [`Component`]s:
/// * [`RectTransform`]
/// * [`Button`]
/// * [`Image`]
/// * [`Text`]
/// * [`Slider`]
/// * [`Scrollbar`]
///
/// [`Stage`]s:
/// * [`UiStage`]
///
/// [`Resource`]:
/// * [`UiResource`]
///
/// # Examples
/// ```
/// # use qinetic_app::prelude::*;
/// # use qinetic_ui::prelude::*;
/// #
/// App::builder().with_plugin(UiPlugin::default()).build().run();
/// ```
#[derive(Default)]
pub struct UiPlugin {}

impl Plugin for UiPlugin {
    fn build(&mut self, app_builder: &mut AppBuilder) {
        app_builder.with_component(RectTransform::default());
        app_builder.with_component(Button::default());
        app_builder.with_component(Image::default());
        app_builder.with_component(Text::default());
        app_builder.with_component(Slider::default());
        app_builder.with_component(Scrollbar::default());
        app_builder.with_resource(UiResource::default());
        app_builder.with_stage(UiStage::default());
    }
}

/// User-interface [`Resource`] for [`App`].
///
/// # Examples
/// ```
/// # use qinetic_app::prelude::*;
/// # use qinetic_ui::prelude::*;
/// #
/// App::builder()
///     .with_plugin(UiPlugin)
///     .with_resource(UiResource::default())
///     .build()
///     .run();
/// ```
#[derive(Resource)]
pub struct UiResource {
    /// The requested user-interface client opacity.
    pub opacity: f32,

    /// The requested user-interface client theme.
    pub theme: Theme,

    /// The requested user-interface client font.
    pub font: Font,

    /// The requested user-interface client window padding.
    pub window_padding: (f32, f32),

    /// The requested user-interface client window rounding.
    pub window_rounding: f32,

    /// The requested user-interface client window border size.
    pub window_border_size: f32,

    /// The requested user-interface client window's child padding.
    pub child_window_padding: (f32, f32),

    /// The requested user-interface client window's child rounding.
    pub child_window_rounding: f32,

    /// The requested user-interface client window's child border size.
    pub child_window_border_size: f32,

    /// The requested user-interface client frame padding.
    pub frame_padding: (f32, f32),

    /// The requested user-interface client frame rounding.
    pub frame_rounding: f32,

    /// The requested user-interface client frame border size.
    pub frame_border_size: f32,

    /// The requested user-interface client inner window items spacing.
    pub item_spacing: (f32, f32),

    /// The requested user-interface client inner window items inner spacing.
    pub item_inner_spacing: (f32, f32),

    /// The requested user-interface client scrollbar thickness.
    pub scrollbar_thickness: f32,

    /// The requested user-interface client scrollbar rounding.
    pub scrollbar_rounding: f32,

    /// The requested user-interface client grab size.
    pub grab_size: f32,

    /// The requested user-interface client grab rounding.
    pub grab_rounding: f32,
}

impl Default for UiResource {
    fn default() -> Self {
        Self {
            opacity: 1.0,

            theme: Theme::default(),

            font: Font::default(),

            window_padding: (10.0, 10.0),
            window_rounding: 5.0,
            window_border_size: 1.0,

            child_window_padding: (15.0, 15.0),
            child_window_rounding: 5.0,
            child_window_border_size: 1.0,

            frame_padding: (5.0, 2.5),
            frame_rounding: 2.5,
            frame_border_size: 0.0,

            item_spacing: (5.0, 5.0),
            item_inner_spacing: (5.0, 5.0),

            scrollbar_thickness: 12.5,
            scrollbar_rounding: 5.0,

            grab_size: 10.0,
            grab_rounding: 2.5,
        }
    }
}

/// User-interface [`Stage`] for [`App`].
///
/// # Examples
/// ```
/// # use qinetic_app::prelude::*;
/// # use qinetic_window:prelude::*;
///`#
/// App::builder().with_stage(Uitage).build().run();
/// ```
#[derive(Default, Stage)]
pub struct UiStage;

/// Rectangle Transform [`Component`].
///
/// # Examples
/// ```
/// # use qinetic_app::prelude::*;
/// # use qinetic_ui::prelude::*;
/// #
/// App::builder().with_component(RectTransform::default()).build().run();
/// ```
#[derive(Default, Component)]
pub struct RectTransform {
    /// width, height in `ui`.
    pub ui_scale: Vec2,

    /// min anchor in `ui`.
    pub anchor_min: Vec2,

    /// max anchor in `ui`.
    pub anchors_max: Vec2,

    /// pivot in `ui`.
    pub pivot: Vec2,

    /// position in [`World`].
    pub position: Vec2,

    /// rotation in [`World`].
    pub rotation: Vec2,

    /// scale in [`World`].
    pub scale: Vec2,
}

/// Button [`Component`].
///
/// # Examples
/// ```
/// # use qinetic_app::prelude::*;
/// # use qinetic_ui::prelude::*;
/// #
/// App::builder().with_component(Button::default()).build().run();
/// ```
#[derive(Default, Component)]
pub struct Button {}

/// Image [`Component`].
///
/// # Examples
/// ```
/// # use qinetic_app::prelude::*;
/// # use qinetic_ui::prelude::*;
/// #
/// App::builder().with_component(Image::default()).build().run();
/// ```
#[derive(Default, Component)]
pub struct Image {}

/// Text [`Component`].
///
/// # Examples
/// ```
/// # use qinetic_app::prelude::*;
/// # use qinetic_ui::prelude::*;
/// #
/// App::builder().with_component(Text::default()).build().run();
/// ```
#[derive(Default, Component)]
pub struct Text {}

/// Slider [`Component`].
///
/// # Examples
/// ```
/// # use qinetic_app::prelude::*;
/// # use qinetic_ui::prelude::*;
/// #
/// App::builder().with_component(Slider::default()).build().run();
/// ```
#[derive(Default, Component)]
pub struct Slider {}

/// Scrollbar [`Component`].
///
/// # Examples
/// ```
/// # use qinetic_app::prelude::*;
/// # use qinetic_ui::prelude::*;
/// #
/// App::builder().with_component(Scrollbar::default()).build().run();
/// ```
#[derive(Default, Component)]
pub struct Scrollbar {}
