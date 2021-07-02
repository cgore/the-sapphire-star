use bevy::prelude::*;

fn startup() {
    println!("Starting The Sapphire Star ...");
}

fn main() {
    App::build()
        .add_plugins(DefaultPlugins)
        .add_startup_system(startup.system())
        .run();
}
