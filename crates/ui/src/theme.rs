use qinetic_core::color4::*;

/// A container of theme data.
pub struct Theme {}

impl Theme {
    /// Returns a [`ThemeBuilder`] with `default` configuration.
    pub fn builder() -> ThemeBuilder {
        ThemeBuilder::default()
    }
}

impl Default for Theme {
    fn default() -> Self {
        let bg1: Color4 = Color4::from_hex("#282828aa").unwrap();
        let bg0: Color4 = Color4::from_hex("#1d2021aa").unwrap();
        let fg0: Color4 = Color4::from_hex("#928374aa").unwrap();
        let fg1: Color4 = Color4::from_hex("#ebdbb2aa").unwrap();

        Self::builder()
            .with_text(fg1)
            .with_text_disabled(fg0)
            .with_text_selected_bg(fg1)
            .with_text_selected(bg1)
            .with_window_bg(bg1)
            .with_window_border(fg1)
            .with_window_shadow(Color4::from_hex("#00000000").unwrap())
            .with_child_window_bg(bg1)
            .with_child_window_border(fg1)
            .with_child_window_shadow(Color4::from_hex("#00000000").unwrap())
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
    }
}

/// A `Builder Pattern` for [`Theme`].
#[derive(Default)]
pub struct ThemeBuilder {
    text: Option<Color4>,
    text_disabled: Option<Color4>,

    text_selected_bg: Option<Color4>,
    text_selected: Option<Color4>,

    window_bg: Option<Color4>,
    window_border: Option<Color4>,
    window_shadow: Option<Color4>,

    child_window_bg: Option<Color4>,
    child_window_border: Option<Color4>,
    child_window_shadow: Option<Color4>,

    frame_bg: Option<Color4>,
    frame_bg_hovered: Option<Color4>,
    frame_bg_active: Option<Color4>,

    title_bg: Option<Color4>,
    title_bg_collapsed: Option<Color4>,
    title_bg_active: Option<Color4>,

    scrollbar_bg: Option<Color4>,
    scrollbar_grab: Option<Color4>,
    scrollbar_hovered: Option<Color4>,
    scrollbar_active: Option<Color4>,

    combo_bg: Option<Color4>,

    check_mark: Option<Color4>,

    slider_grab: Option<Color4>,
    slider_grab_active: Option<Color4>,

    button: Option<Color4>,
    button_hovered: Option<Color4>,
    button_active: Option<Color4>,

    header: Option<Color4>,
    header_hovered: Option<Color4>,
    header_active: Option<Color4>,

    close_button: Option<Color4>,
    close_button_hovered: Option<Color4>,
    close_button_active: Option<Color4>,

    resize_grip: Option<Color4>,
    resize_grip_hovered: Option<Color4>,
    resize_grip_active: Option<Color4>,
}

impl ThemeBuilder {
    /// Returns a [`ThemeBuilder`] with `text` color.
    #[inline]
    pub fn with_text(mut self, color: Color4) -> Self {
        self.text = Some(color);
        self
    }

    /// Returns a [`ThemeBuilder`] with `text disabled` color.
    #[inline]
    pub fn with_text_disabled(mut self, color: Color4) -> Self {
        self.text_disabled = Some(color);
        self
    }

    /// Returns a [`ThemeBuilder`] with `text selected background` color.
    #[inline]
    pub fn with_text_selected_bg(mut self, color: Color4) -> Self {
        self.text_selected_bg = Some(color);
        self
    }

    /// Returns a [`ThemeBuilder`] with `text selected` color.
    #[inline]
    pub fn with_text_selected(mut self, color: Color4) -> Self {
        self.text_selected = Some(color);
        self
    }

    /// Returns a [`ThemeBuilder`] with `window background` color.
    #[inline]
    pub fn with_window_bg(mut self, color: Color4) -> Self {
        self.window_bg = Some(color);
        self
    }

    /// Returns a [`ThemeBuilder`] with `window border` color.
    #[inline]
    pub fn with_window_border(mut self, color: Color4) -> Self {
        self.window_border = Some(color);
        self
    }

    /// Returns a [`ThemeBuilder`] with `window shadow` color.
    #[inline]
    pub fn with_window_shadow(mut self, color: Color4) -> Self {
        self.window_shadow = Some(color);
        self
    }

    /// Returns a [`ThemeBuilder`] with `child window background` color.
    #[inline]
    pub fn with_child_window_bg(mut self, color: Color4) -> Self {
        self.child_window_bg = Some(color);
        self
    }

    /// Returns a [`ThemeBuilder`] with `child window border` color.
    #[inline]
    pub fn with_child_window_border(mut self, color: Color4) -> Self {
        self.child_window_border = Some(color);
        self
    }

    /// Returns a [`ThemeBuilder`] with `child window shadow` color.
    #[inline]
    pub fn with_child_window_shadow(mut self, color: Color4) -> Self {
        self.child_window_shadow = Some(color);
        self
    }

    /// Returns a [`ThemeBuilder`] with `frame background` color.
    #[inline]
    pub fn with_frame_bg(mut self, color: Color4) -> Self {
        self.frame_bg = Some(color);
        self
    }

    /// Returns a [`ThemeBuilder`] with `frame background hovered` color.
    #[inline]
    pub fn with_frame_bg_hovered(mut self, color: Color4) -> Self {
        self.frame_bg_hovered = Some(color);
        self
    }

    /// Returns a [`ThemeBuilder`] with `frame background active` color.
    #[inline]
    pub fn with_frame_bg_active(mut self, color: Color4) -> Self {
        self.frame_bg_active = Some(color);
        self
    }

    /// Returns a [`ThemeBuilder`] with `title background` color.
    #[inline]
    pub fn with_title_bg(mut self, color: Color4) -> Self {
        self.title_bg = Some(color);
        self
    }

    /// Returns a [`ThemeBuilder`] with `title background collapsed` color.
    #[inline]
    pub fn with_title_bg_collapsed(mut self, color: Color4) -> Self {
        self.title_bg_collapsed = Some(color);
        self
    }

    /// Returns a [`ThemeBuilder`] with `title background active` color.
    #[inline]
    pub fn with_title_bg_active(mut self, color: Color4) -> Self {
        self.title_bg_active = Some(color);
        self
    }

    /// Returns a [`ThemeBuilder`] with `scrollbar background` color.
    #[inline]
    pub fn with_scrollbar_bg(mut self, color: Color4) -> Self {
        self.scrollbar_bg = Some(color);
        self
    }

    /// Returns a [`ThemeBuilder`] with `scrollbar grab` color.
    #[inline]
    pub fn with_scrollbar_grab(mut self, color: Color4) -> Self {
        self.scrollbar_grab = Some(color);
        self
    }

    /// Returns a [`ThemeBuilder`] with `scrollbar hovered` color.
    #[inline]
    pub fn with_scrollbar_hovered(mut self, color: Color4) -> Self {
        self.scrollbar_hovered = Some(color);
        self
    }

    /// Returns a [`ThemeBuilder`] with `scrollbar active` color.
    #[inline]
    pub fn with_scrollbar_active(mut self, color: Color4) -> Self {
        self.scrollbar_active = Some(color);
        self
    }

    /// Returns a [`ThemeBuilder`] with `combo background` color.
    #[inline]
    pub fn with_combo_bg(mut self, color: Color4) -> Self {
        self.combo_bg = Some(color);
        self
    }

    /// Returns a [`ThemeBuilder`] with `check mark` color.
    #[inline]
    pub fn with_check_mark(mut self, color: Color4) -> Self {
        self.check_mark = Some(color);
        self
    }

    /// Returns a [`ThemeBuilder`] with `slider grab` color.
    #[inline]
    pub fn with_slider_grab(mut self, color: Color4) -> Self {
        self.slider_grab = Some(color);
        self
    }

    /// Returns a [`ThemeBuilder`] with `slider grab active` color.
    #[inline]
    pub fn with_slider_grab_active(mut self, color: Color4) -> Self {
        self.slider_grab_active = Some(color);
        self
    }

    /// Returns a [`ThemeBuilder`] with `button` color.
    #[inline]
    pub fn with_button(mut self, color: Color4) -> Self {
        self.button = Some(color);
        self
    }

    /// Returns a [`ThemeBuilder`] with `button hovered` color.
    #[inline]
    pub fn with_button_hovered(mut self, color: Color4) -> Self {
        self.button_hovered = Some(color);
        self
    }

    /// Returns a [`ThemeBuilder`] with `button active` color.
    #[inline]
    pub fn with_button_active(mut self, color: Color4) -> Self {
        self.button_active = Some(color);
        self
    }

    /// Returns a [`ThemeBuilder`] with `header background` color.
    #[inline]
    pub fn with_header(mut self, color: Color4) -> Self {
        self.header = Some(color);
        self
    }

    /// Returns a [`ThemeBuilder`] with `header background hovered` color.
    #[inline]
    pub fn with_header_hovered(mut self, color: Color4) -> Self {
        self.header_hovered = Some(color);
        self
    }

    /// Returns a [`ThemeBuilder`] with `header background active` color.
    #[inline]
    pub fn with_header_active(mut self, color: Color4) -> Self {
        self.header_active = Some(color);
        self
    }

    /// Returns a [`ThemeBuilder`] with `close button` color.
    #[inline]
    pub fn with_close_button(mut self, color: Color4) -> Self {
        self.close_button = Some(color);
        self
    }

    /// Returns a [`ThemeBuilder`] with `close button hovered` color.
    #[inline]
    pub fn with_close_button_hovered(mut self, color: Color4) -> Self {
        self.close_button_hovered = Some(color);
        self
    }

    /// Returns a [`ThemeBuilder`] with `close button active` color.
    #[inline]
    pub fn with_close_button_active(mut self, color: Color4) -> Self {
        self.close_button_active = Some(color);
        self
    }

    /// Returns a [`ThemeBuilder`] with `resize grip` color.
    #[inline]
    pub fn with_resize_grip(mut self, color: Color4) -> Self {
        self.resize_grip = Some(color);
        self
    }

    /// Returns a [`ThemeBuilder`] with `resize grip hovered` color.
    #[inline]
    pub fn with_resize_grip_hovered(mut self, color: Color4) -> Self {
        self.resize_grip_hovered = Some(color);
        self
    }

    /// Returns a [`ThemeBuilder`] with `resize grip active` color.
    #[inline]
    pub fn with_resize_grip_active(mut self, color: Color4) -> Self {
        self.resize_grip_active = Some(color);
        self
    }

    /// Returns a [`Theme`] configured from [`ThemeBuilder`].
    pub fn build(mut self) -> Theme {
        todo!()
    }
}
