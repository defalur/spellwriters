use bevy::prelude::*;
use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;
use std::env::current_dir;

use crate::resources::ResourceHandles;

pub struct ArenaPlugin;

pub struct Tile {
    wall: bool,
}

pub struct Arena {
    width: usize,
    height: usize,
    tiles: Vec<Entity>,
}

impl Plugin for ArenaPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_startup_system(load_map.system());
    }
}

fn load_map(mut commands: Commands,
            resources: Res<ResourceHandles>,
            texture_atlases: Res<Assets<TextureAtlas>>,) {
    //load map
    println!("Loading map");
    let file = File::open("assets/map");
    let w = 16;
    let h = 16;
    let mut arena = Arena{width: 16, height: 16, tiles: Vec::new()};
    if let Ok(f) = file {
        let mut buf_reader = BufReader::new(f);
        let mut contents = String::new();
        if let Ok(_) = buf_reader.read_to_string(&mut contents) {
            println!("{:?}", resources.ground_sheet);
            let mut i = 0;
            for c in contents.chars() {
                if c == '\n' {
                    continue;
                }
                let tile = Entity::new();
                arena.tiles.push(tile);
                commands
                    .spawn_as_entity(
                        tile,
                        SpriteSheetComponents {
                            sprite: TextureAtlasSprite::new((c == 'x') as u32),
                            texture_atlas: resources.ground_sheet,
                            translation: Translation(
                                Vec3::new(((i % w) * 16) as f32, ((i / h) * 16) as f32, 0.0)),
                            ..Default::default()
                        })
                    .with(Tile{wall: c == 'x'});
                i += 1;
            }
            println!("Finished loading map");
        }
        else {
            println!("Failed reading map file");
        }
    }
    else {
        println!("Map file not found");
    }
}
