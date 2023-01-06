//! Theme creation and loading.

use qinetic_utils::prelude::*;

/// A container of theme data.
///
/// # Examples
/// ```
/// # use qinetic_ui::prelude::*;
/// # use qinetic_utils::prelude::*;
/// #
/// let theme = Theme::builder()
///     .with_window_bg(RGBA8::new(40, 40, 40, 255))
///     .build()
///     .unwrap();
/// ```
#[derive(SmartDefault, Clone, Copy, Debug, PartialEq, Eq, Getters, Builder)]
#[getset(get = "pub")]
#[builder(
    crate = "crate::theme",
    setter(prefix = "with"),
    default,
    derive(Debug, PartialEq, Eq)
)]
pub struct Theme {
    #[default(RGBA8::new(235, 219, 178, 255))]
    text: RGBA8,

    #[default(RGBA8::new(146, 131, 116, 255))]
    text_disabled: RGBA8,

    #[default(RGBA8::new(235, 219, 178, 255))]
    text_selected_bg: RGBA8,

    #[default(RGBA8::new(40, 40, 40, 255))]
    text_selected: RGBA8,

    #[default(RGBA8::new(40, 40, 40, 255))]
    window_bg: RGBA8,

    #[default(RGBA8::new(235, 219, 178, 255))]
    window_border: RGBA8,

    #[default(RGBA8::new(0, 0, 0, 255))]
    window_shadow: RGBA8,

    #[default(RGBA8::new(40, 40, 40, 255))]
    child_window_bg: RGBA8,

    #[default(RGBA8::new(235, 219, 178, 255))]
    child_window_border: RGBA8,

    #[default(RGBA8::new(0, 0, 0, 255))]
    child_window_shadow: RGBA8,

    #[default(RGBA8::new(40, 40, 40, 255))]
    frame_bg: RGBA8,

    #[default(RGBA8::new(146, 131, 116, 255))]
    frame_bg_hovered: RGBA8,

    #[default(RGBA8::new(235, 219, 178, 255))]
    frame_bg_active: RGBA8,

    #[default(RGBA8::new(40, 40, 40, 255))]
    title_bg: RGBA8,

    #[default(RGBA8::new(40, 40, 40, 255))]
    title_bg_collapsed: RGBA8,

    #[default(RGBA8::new(40, 40, 40, 255))]
    title_bg_active: RGBA8,

    #[default(RGBA8::new(40, 40, 40, 255))]
    scrollbar_bg: RGBA8,

    #[default(RGBA8::new(235, 219, 178, 255))]
    scrollbar_grab: RGBA8,

    #[default(RGBA8::new(146, 131, 116, 255))]
    scrollbar_hovered: RGBA8,

    #[default(RGBA8::new(235, 219, 178, 255))]
    scrollbar_active: RGBA8,

    #[default(RGBA8::new(29, 32, 33, 255))]
    combo_bg: RGBA8,

    #[default(RGBA8::new(235, 219, 178, 255))]
    check_mark: RGBA8,

    #[default(RGBA8::new(146, 131, 116, 255))]
    slider_grab: RGBA8,

    #[default(RGBA8::new(235, 219, 178, 255))]
    slider_grab_active: RGBA8,

    #[default(RGBA8::new(29, 32, 33, 255))]
    button: RGBA8,

    #[default(RGBA8::new(146, 131, 116, 255))]
    button_hovered: RGBA8,

    #[default(RGBA8::new(235, 219, 178, 255))]
    button_active: RGBA8,

    #[default(RGBA8::new(40, 40, 40, 255))]
    header: RGBA8,

    #[default(RGBA8::new(235, 219, 178, 255))]
    header_hovered: RGBA8,

    #[default(RGBA8::new(235, 219, 178, 255))]
    header_active: RGBA8,

    #[default(RGBA8::new(235, 219, 178, 255))]
    close_button: RGBA8,

    #[default(RGBA8::new(146, 131, 116, 255))]
    close_button_hovered: RGBA8,

    #[default(RGBA8::new(60, 56, 54, 255))]
    close_button_active: RGBA8,

    #[default(RGBA8::new(60, 56, 54, 255))]
    resize_grip: RGBA8,

    #[default(RGBA8::new(146, 131, 116, 255))]
    resize_grip_hovered: RGBA8,

    #[default(RGBA8::new(235, 219, 178, 255))]
    resize_grip_active: RGBA8,
}

impl Theme {
    /// Returns a [`ThemeBuilder`] with `default` configuration.
    ///
    /// # Examples
    /// ```
    /// # use qinetic_ui::prelude::*;
    /// #
    /// let theme_builder = Theme::builder();
    /// ```
    #[inline]
    pub fn builder() -> ThemeBuilder {
        ThemeBuilder::default()
    }
}
