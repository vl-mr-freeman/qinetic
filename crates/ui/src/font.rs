use std::fs::File;

/// A conteiner of font data.
pub struct Font {}

impl Font {
    /// Returns a [`FontBuilder`] with `default` configuration.
    pub fn builder() -> FontBuilder {
        FontBuilder::default()
    }
}

impl Default for Font {
    fn default() -> Self {
        #[cfg(target_family = "unix")]
        FontBuilder::default()
            .with_file_ttf("notosans.ttf")
            .with_size(12.0)
            .build()
    }
}

/// A `Builder Pattern` for [`Font`].
pub struct FontBuilder {
    path: Option<String>,
    size: Option<f32>,
}

impl FontBuilder {
    /// Returns a [`FontBuilder`] with path to [`Font`] .ttf file.
    pub fn with_file_ttf(mut self, path: &str) -> Self {
        self.path = Some(path.into());
        self
    }

    /// Returns a [`FontBuilder`] with path to [`Font`] .otf file.
    pub fn with_file_otf(mut self, path: &str) -> Self {
        self.path = Some(path.into());
        self
    }

    /// Returns a [`FontBuilder`] with size of `font`.
    #[inline]
    pub fn with_size(mut self, size: f32) -> Self {
        self.size = Some(size);
        self
    }

    /// Returns a [`Font`] configured from [`FontBuilder`].
    pub fn build(mut self) -> Font {
        todo!()
    }
}

impl Default for FontBuilder {
    fn default() -> Self {
        Self {
            path: None,
            size: None,
        }
    }
}
