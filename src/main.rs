mod creatures;
mod startup;

use bevy::{prelude::App, DefaultPlugins};

use creatures::EntityPlugin;
use startup::StartupPlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(StartupPlugin)
        .add_plugin(EntityPlugin)
        .run();
}
