use bevy::prelude::*;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, _app: &mut App) {
        println!("Hello world!");
    }
}