use bevy::prelude::*;

// Resources
pub const CLEAR: Color = Color::rgb(0.1, 0.1, 0.1);

fn main() {
    App::new()
        .insert_resource(ClearColor(CLEAR))
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                resolution: (900., 500.).into(),
                title: "bevy_test".to_string(),
                ..default()
            }),
            ..default()
        }))
        .run();
}
