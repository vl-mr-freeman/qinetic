use qinetic::prelude::*;

fn main() {
    App::builder()
        .with_plugins(DefaultPlugins::default())
        .with_runner(RunOnce::default())
        .build()
        .run();
}
