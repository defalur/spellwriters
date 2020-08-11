use bevy::prelude::*;

mod arena;
mod resources;

fn main() {
    App::build()
        .add_resource(WindowDescriptor {
            title: "SpellWriters!".to_string(),
            width: 800,
            height: 600,
            vsync: true,
        })
        .add_default_plugins()
        .add_plugin(resources::ResourcePlugin)
        .add_plugin(arena::ArenaPlugin)
        .add_startup_system(setup.system())
        .run();
}

fn setup(mut commands: Commands) {
    commands
        .spawn(Camera2dComponents::default());
}
