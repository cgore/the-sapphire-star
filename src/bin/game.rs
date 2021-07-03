use bevy::prelude::*;
use the_sapphire_star::startup;

fn main() {
    App::build()
        .insert_resource(Msaa { samples: 4 })
        .add_plugins(DefaultPlugins)
        .add_startup_system(startup::startup.system())
        .run();
}
