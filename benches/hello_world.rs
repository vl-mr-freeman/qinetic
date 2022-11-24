use qinetic::prelude::*;

fn main() {
    App::builder()
        .with_plugins(DefaultPlugins)
        .with_runner(runner)
        .build()
        .run();
}

fn runner(mut app: App) {
    println!("[runner]: hello, world!");
    app.update();
}
