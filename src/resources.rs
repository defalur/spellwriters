use bevy::prelude::*;

pub struct ResourcePlugin;

#[derive(Default)]
pub struct ResourceHandles {
    pub ground_sheet: Handle<TextureAtlas>,
}

impl Plugin for ResourcePlugin {
    fn build(&self, app: &mut AppBuilder) {
        app
            .add_resource(ResourceHandles::default())
            .add_startup_system(setup.system());
    }
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut textures: ResMut<Assets<Texture>>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    mut resources: ResMut<ResourceHandles>,
) {
    println!("Loading textures.");
    let texture_handle = asset_server
        .load_sync(
            &mut textures,
            "assets/textures/ground_1.png",
        )
        .unwrap();
    let texture = textures.get(&texture_handle).unwrap();
    let texture_atlas = TextureAtlas::from_grid(texture_handle, texture.size, 40, 36);

    resources.ground_sheet = texture_atlases.add(texture_atlas);
    println!("{:?}", resources.ground_sheet);
}
