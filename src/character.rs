use bevy::prelude::*;

use crate::resources::ResourceHandles;
use crate::arena::GridTransform;

pub struct Character;

//a character contains a sprite and a position on the grid

pub struct CharacterPlugin;

impl Plugin for CharacterPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app
            .add_startup_system(add_char.system());
    }
}

fn add_char(mut commands: Commands,
            resources: Res<ResourceHandles>,) {
    println!("Loading char.");
    let pos = GridTransform{x: 2, y: 5};

    let (x_pix, y_pix) = pos.to_pix();

    println!("{:?}", resources.char_sheet);
    commands.spawn(
        SpriteSheetComponents {
            texture_atlas: resources.char_sheet,
            translation: Translation(
                Vec3::new(x_pix, y_pix, 0.1)),
            scale: Scale(2.0),
            ..Default::default()
        })
    .with(Character)
    .with(pos);
}
