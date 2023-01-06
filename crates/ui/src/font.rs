//! Font loading.

use qinetic_utils::prelude::*;

/// A conteiner of font data.
///
/// # Examples
/// ```
/// # use qinetic_ui::prelude::*;
/// # use qinetic_utils::prelude::*;
/// #
/// let font = Font::builder()
///     .with_path("FiraCode.ttf")
///     .with_size(14.0)
///     .build()
///     .unwrap();
/// ```
#[derive(SmartDefault, Clone, Debug, PartialEq, Getters, Builder)]
#[getset(get = "pub")]
#[builder(
    crate = "crate::font",
    setter(prefix = "with"),
    default,
    derive(Debug, PartialEq)
)]
pub struct Font {
    #[builder(setter(into))]
    #[default = "notosans.ttf"]
    path: String,

    #[default = 12.0]
    size: f32,
}

impl Font {
    /// Returns a [`FontBuilder`] with `default` configuration.
    ///
    /// # Examples
    /// ```
    /// # use qinetic_ui::prelude::*;
    /// #
    /// let font_builder = Font::builder();
    /// ```
    #[inline]
    pub fn builder() -> FontBuilder {
        FontBuilder::default()
    }
}
