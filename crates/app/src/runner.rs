use crate::app::App;
use std::any::Any;

/// Runner that calls in [App::run].
pub trait Runner: Any + 'static {
    fn run(&mut self, app: App);
}
