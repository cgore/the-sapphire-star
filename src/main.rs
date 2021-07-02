use bevy::prelude::*;

mod startup;

fn main() {
    App::build()
        .add_plugins(DefaultPlugins)
        .add_startup_system(startup::startup.system())
        .run();
}
