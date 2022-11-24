use qinetic::prelude::*;

const WIDTH: u32 = 1280;
const HEIGHT: u32 = 720;

fn main() {
    App::builder()
        .with_runner(runner)
        .with_resource(WindowResource {
            width: WIDTH,
            height: HEIGHT,
            title: "Qinetic Hello World Example".to_string(),
            ..Default::default()
        })
        .with_resource(RenderResource {
            ..Default::default()
        })
        .with_plugins(DefaultPlugins)
        .build()
        .run();
}

fn runner(mut app: App) {
    loop {
        println!("[runner]: hello, world!");
        app.update();
    }
}
