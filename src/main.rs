mod game;
mod player;

use bevy::prelude::*;
use bevy_editor_pls::prelude::*;
use game::GamePlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .add_plugins(EditorPlugin::default())
        .add_plugins(GamePlugin)
        .run();
}
