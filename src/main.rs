use bevy::{prelude::*, render::camera::ScalingMode};

// Resources
pub const CLEAR: Color = Color::rgb(0.1, 0.1, 0.1);
pub const RESOLUTION: f32 = 16. / 9.;

fn main() {
    let height: f32 = 400.;

    App::new()
        .insert_resource(ClearColor(CLEAR))
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                resolution: (height*RESOLUTION, height).into(),
                title: "bevy_test".to_string(),
                ..default()
            }),
            ..default()
        }))
        .add_systems(spawn_camera)
        .add_systems(spawn_player)
        .add_systems(PreStartup, load_ascii)
        .run();
}

fn spawn_player(mut commands: Commands) {
    let ascii = AsciiSheet;
    let mut sprite = TextureAtlasSprite::new(1);
    sprite.color = Color::rgb(0.3, 0.3, 0.9);
    sprite.custom_size = Some(Vec2::splat(1.0));

    let player = commands
        .spawn(SpriteSheetBundle {
            sprite: sprite,
            texture_atlas: ascii.0.clone(),
            transform: Transform {
                translation: Vec3::new(0.0, 0.0, 900.0),
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(Name::new("Player"))
        .id();

    let mut background_sprite = TextureAtlasSprite::new(0);
    background_sprite.color = Color::rgb(0.5, 0.5, 0.5);
    background_sprite.custom_size = Some(Vec2::splat(1.0));

    let background = commands
        .spawn(SpriteSheetBundle {
            sprite: background_sprite,
            texture_atlas: ascii.0.clone(),
            transform: Transform {
                translation: Vec3::new(0.0, 0.0, -1.0),
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(Name::new("Background"))
        .id(); //id() gives back the entity after creation

    commands.entity(player).push_children(&[background]);
}

fn spawn_camera(mut commands: Commands) {
    let mut camera = Camera2dBundle {
        projection: OrthographicProjection {
            scaling_mode: ScalingMode::FixedVertical(1.),
            ..default()
        }.into(),
        ..default()
    };

    //camera.orthographic_projection.left = -1. * RESOLUTION;
    //camera.orthographic_projection.right = 1. * RESOLUTION;

    commands.spawn(camera);
}

struct AsciiSheet(Handle<TextureAtlas>);

fn load_ascii(
    mut commands:Commands,
    assets: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>> ) {
        let image = assets.load("Ascii.png");
        let atlas = TextureAtlas::from_grid_with_padding(
                image,
                Vec::splat(9.),
                16,
                16,
                Vec2::splat(2.)
        ); 
        let atlas_handle = texture_atlases.add(atlas);

        commands.insert_resource(AsciiSheet(atlas_handle));
    }
