use qinetic::prelude::*;

fn main() {
    App::new().add_plugin_group(DefaultPluginGroup).run();
}

fn hello_world_system() {
    println!("hello, world!");
}
