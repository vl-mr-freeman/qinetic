use qinetic::prelude::*;

fn main() {
    App::builder()
        .with_runner(RunOnce::default())
        .build()
        .unwrap()
        .run();
}
