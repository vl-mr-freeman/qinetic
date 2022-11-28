use qinetic_app::app::App;
use qinetic_app::runner::*;

/// [`Runner`] for [`App`] that calls once.
#[derive(Default)]
pub struct RunOnce;

impl Runner for RunOnce {
    fn run(&mut self, mut app: App) {
        app.update();
    }
}

/// [`Runner`] for [`App`] that calls in loop.
#[derive(Default)]
pub struct RunLoop;

impl Runner for RunLoop {
    fn run(&mut self, mut app: App) {
        loop {
            app.update();
        }
    }
}
