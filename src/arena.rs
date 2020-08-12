use bevy::prelude::*;
use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;
use std::env::current_dir;

use crate::resources::ResourceHandles;

pub struct ArenaPlugin;

pub struct Tile;

#[derive(Default)]
pub struct Arena {
    width: usize,
    height: usize,
    obstacles: Vec<bool>,
    tiles: Vec<Entity>,
}

pub struct GridTransform {
    pub x: i32,
    pub y: i32,
}

impl GridTransform {
    pub fn to_pix(&self) -> (f32, f32) {
        (self.x as f32 * 32.0, self.y as f32 * 32.0)
    }
}

impl Plugin for ArenaPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app
            .add_resource(Arena::default())
            .add_startup_system(load_map.system());
    }
}

fn load_map(mut commands: Commands,
            resources: Res<ResourceHandles>,
            texture_atlases: Res<Assets<TextureAtlas>>,
            mut arena_res: ResMut<Arena>,) {
    //load map
    println!("Loading map");
    let file = File::open("assets/map");
    let w = 16;
    let h = 16;

    arena_res.width = w;
    arena_res.height = h;

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
                arena_res.tiles.push(tile);
                arena_res.obstacles.push(c == 'x');
                commands
                    .spawn_as_entity(
                        tile,
                        SpriteSheetComponents {
                            sprite: TextureAtlasSprite::new((c == 'x') as u32),
                            texture_atlas: resources.ground_sheet,
                            translation: Translation(
                                Vec3::new(((i % w) * 32) as f32, ((i / h) * 32) as f32, 0.0)),
                            scale: Scale(2.0),
                            ..Default::default()
                        })
                    .with(Tile);
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
