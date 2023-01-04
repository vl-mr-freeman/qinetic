//! Theme creation and loading.

use qinetic_utils::prelude::*;

/// A container of theme data.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Getters, Builder)]
#[getset(get = "pub")]
#[builder(setter(prefix = "with"), default, derive(Debug, PartialEq, Eq))]
pub struct Theme {
    //#[default(RGBA8::new(235, 219, 178, 255))]
    text: RGBA8,

    //#[default(_code = "RGBA8::new(146, 131, 116, 255)")]
    text_disabled: RGBA8,

    //#[default(_code = "RGBA8::new(235, 219, 178, 255)")]
    text_selected_bg: RGBA8,

    //#[default(_code = "RGBA8::new(40, 40, 40, 255)")]
    text_selected: RGBA8,

    //#[default(_code = "RGBA8::new(40, 40, 40, 255)")]
    window_bg: RGBA8,

    //#[default(_code = "RGBA8::new(235, 219, 178, 255)")]
    window_border: RGBA8,

    //#[default(_code = "RGBA8::new(0, 0, 0, 255)")]
    window_shadow: RGBA8,

    //#[default(_code = "RGBA8::new(40, 40, 40, 255)")]
    child_window_bg: RGBA8,

    //#[default(_code = "RGBA8::new(235, 219, 178, 255)")]
    child_window_border: RGBA8,

    //#[default(_code = "RGBA8::new(0, 0, 0, 255)")]
    child_window_shadow: RGBA8,

    //#[default(_code = "RGBA8::new(40, 40, 40, 255)")]
    frame_bg: RGBA8,

    //#[default(_code = "RGBA8::new(146, 131, 116, 255)")]
    frame_bg_hovered: RGBA8,

    //#[default(_code = "RGBA8::new(235, 219, 178, 255)")]
    frame_bg_active: RGBA8,

    //#[default(_code = "RGBA8::new(40, 40, 40, 255)")]
    title_bg: RGBA8,

    //#[default(_code = "RGBA8::new(40, 40, 40, 255)")]
    title_bg_collapsed: RGBA8,

    //#[default(_code = "RGBA8::new(40, 40, 40, 255)")]
    title_bg_active: RGBA8,

    //#[default(_code = "RGBA8::new(40, 40, 40, 255)")]
    scrollbar_bg: RGBA8,

    //#[default(_code = "RGBA8::new(235, 219, 178, 255)")]
    scrollbar_grab: RGBA8,

    //#[default(_code = "RGBA8::new(146, 131, 116, 255)")]
    scrollbar_hovered: RGBA8,

    //#[default(_code = "RGBA8::new(235, 219, 178, 255)")]
    scrollbar_active: RGBA8,

    //#[default(_code = "RGBA8::new(29, 32, 33, 255)")]
    combo_bg: RGBA8,

    //#[default(_code = "RGBA8::new(235, 219, 178, 255)")]
    check_mark: RGBA8,

    //#[default(_code = "RGBA8::new(146, 131, 116, 255)")]
    slider_grab: RGBA8,

    //#[default(_code = "RGBA8::new(235, 219, 178, 255)")]
    slider_grab_active: RGBA8,

    //#[default(_code = "RGBA8::new(29, 32, 33, 255)")]
    button: RGBA8,

    //#[default(_code = "RGBA8::new(146, 131, 116, 255)")]
    button_hovered: RGBA8,

    //#[default(_code = "RGBA8::new(235, 219, 178, 255)")]
    button_active: RGBA8,

    //#[default(_code = "RGBA8::new(40, 40, 40, 255)")]
    header: RGBA8,

    //#[default(_code = "RGBA8::new(235, 219, 178, 255)")]
    header_hovered: RGBA8,

    //#[default(_code = "RGBA8::new(235, 219, 178, 255)")]
    header_active: RGBA8,

    //#[default(_code = "RGBA8::new(235, 219, 178, 255)")]
    close_button: RGBA8,

    //#[default(_code = "RGBA8::new(146, 131, 116, 255)")]
    close_button_hovered: RGBA8,

    //#[default(_code = "RGBA8::new(60, 56, 54, 255)")]
    close_button_active: RGBA8,

    //#[default(_code = "RGBA8::new(60, 56, 54, 255)")]
    resize_grip: RGBA8,

    //#[default(_code = "RGBA8::new(146, 131, 116, 255)")]
    resize_grip_hovered: RGBA8,

    //#[default(_code = "RGBA8::new(235, 219, 178, 255)")]
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

impl Default for Theme {
    fn default() -> Self {
        let bg1: RGBA8 = RGBA8::new(40, 40, 40, 255);
        let bg0: RGBA8 = RGBA8::new(29, 32, 33, 255);
        let fg0: RGBA8 = RGBA8::new(124, 131, 116, 255);
        let fg1: RGBA8 = RGBA8::new(235, 219, 178, 255);

        Self::builder()
            .with_text(fg1)
            .with_text_disabled(fg0)
            .with_text_selected_bg(fg1)
            .with_text_selected(bg1)
            .with_window_bg(bg1)
            .with_window_border(fg1)
            .with_window_shadow(RGBA8::new(0, 0, 0, 255))
            .with_child_window_bg(bg1)
            .with_child_window_border(fg1)
            .with_child_window_shadow(RGBA8::new(0, 0, 0, 255))
            .with_frame_bg(bg1)
            .with_frame_bg_hovered(fg0)
            .with_frame_bg_active(fg1)
            .with_title_bg(bg1)
            .with_title_bg_collapsed(bg1)
            .with_title_bg_active(bg1)
            .with_scrollbar_bg(bg1)
            .with_scrollbar_grab(fg1)
            .with_scrollbar_hovered(fg0)
            .with_scrollbar_active(fg1)
            .with_combo_bg(bg0)
            .with_check_mark(fg1)
            .with_slider_grab(fg0)
            .with_slider_grab_active(fg1)
            .with_button(bg0)
            .with_button_hovered(fg0)
            .with_button_active(fg1)
            .with_header(bg1)
            .with_header_hovered(fg0)
            .with_header_active(fg1)
            .with_close_button(fg1)
            .with_close_button_hovered(fg0)
            .build()
            .unwrap()
    }
}
