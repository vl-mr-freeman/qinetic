use qinetic::prelude::*;

fn main() {
    App::builder()
        .with_runner(RunLoop::default())
        .with_plugins(DefaultPlugins::default())
        .build()
        .run();
}
