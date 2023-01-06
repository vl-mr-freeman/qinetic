use qinetic::prelude::*;

fn main() {
    App::builder()
        .with_plugin_group(DefaultPluginGroup::default())
        .with_runner(RunOnce::default())
        .build()
        .unwrap()
        .run();
}
