//! User-interface [`Component`]s functionality.

use qinetic_ecs::prelude::*;
use qinetic_math::prelude::*;
use qinetic_utils::prelude::*;

/// Rectangle Transform [`Component`].
///
/// # Examples
/// ```
/// # use qinetic_app::prelude::*;
/// # use qinetic_ui::prelude::*;
/// #
/// App::builder()
///     .with_component(RectTransform::default())
///     .build()
///     .unwrap();
/// ```
#[derive(SmartDefault, Clone, Copy, Debug, PartialEq, Component)]
pub struct RectTransform {
    /// width, height in `ui`.
    pub ui_scale: Vector2<f32>,

    /// min anchor in `ui`.
    pub anchor_min: Vector2<f32>,

    /// max anchor in `ui`.
    pub anchors_max: Vector2<f32>,

    /// pivot in `ui`.
    pub pivot: Vector2<f32>,

    /// position in [`World`].
    pub position: Vector2<f32>,

    /// rotation in [`World`].
    pub rotation: Vector2<f32>,

    /// scale in [`World`].
    pub scale: Vector2<f32>,
}

/// Button [`Component`].
///
/// # Examples
/// ```
/// # use qinetic_app::prelude::*;
/// # use qinetic_ui::prelude::*;
/// #
/// App::builder()
///     .with_component(Button::default())
///     .build()
///     .unwrap();
/// ```
#[derive(SmartDefault, Clone, Copy, Debug, PartialEq, Eq, Component)]
pub struct Button {}

/// Image [`Component`].
///
/// # Examples
/// ```
/// # use qinetic_app::prelude::*;
/// # use qinetic_ui::prelude::*;
/// #
/// App::builder()
///     .with_component(Image::default())
///     .build()
///     .unwrap();
/// ```
#[derive(SmartDefault, Clone, Copy, Debug, PartialEq, Eq, Component)]
pub struct Image {}

/// Text [`Component`].
///
/// # Examples
/// ```
/// # use qinetic_app::prelude::*;
/// # use qinetic_ui::prelude::*;
/// #
/// App::builder()
///     .with_component(Text::default())
///     .build()
///     .unwrap();
/// ```
#[derive(SmartDefault, Clone, Copy, Debug, PartialEq, Eq, Component)]
pub struct Text {}

/// Slider [`Component`].
///
/// # Examples
/// ```
/// # use qinetic_app::prelude::*;
/// # use qinetic_ui::prelude::*;
/// #
/// App::builder()
///     .with_component(Slider::default())
///     .build()
///     .unwrap();
/// ```
#[derive(SmartDefault, Clone, Copy, Debug, PartialEq, Eq, Component)]
pub struct Slider {}

/// Scrollbar [`Component`].
///
/// # Examples
/// ```
/// # use qinetic_app::prelude::*;
/// # use qinetic_ui::prelude::*;
/// #
/// App::builder()
///     .with_component(Scrollbar::default())
///     .build()
///     .unwrap();
/// ```
#[derive(SmartDefault, Clone, Copy, Debug, PartialEq, Eq, Component)]
pub struct Scrollbar {}
