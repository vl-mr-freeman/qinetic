/// A conteiner of application logic.
pub struct Application {}

impl Default for Application {
    fn default() -> Self {
        let app = Application::empty();

        app
    }
}

impl Application {
    /// Create a new [`Application`] with some default configuration.
    pub fn new() -> Self {
        Self::default()
    }

    /// Create a new [`Application`] with minimal configuration.
    pub fn empty() -> Self {
        Self {}
    }

    /// Starts the [`Application`].
    pub fn run(&mut self) {}
}
