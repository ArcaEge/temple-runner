use bevy::prelude::*;

use crate::player::PlayerPlugin;


pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_camera);
        app.add_plugins(PlayerPlugin);
    }
}

#[derive(Component)]
struct GameCamera;

fn setup_camera(mut commands: Commands) {
    commands.spawn((
        Camera2d::default(),
        GameCamera,
    ));
}