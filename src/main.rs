use bevy::prelude::*;
use bevy_editor_pls::prelude::*;

mod game;
use game::GamePlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(EditorPlugin::default())
        .add_plugins(GamePlugin)
        .run();
}
