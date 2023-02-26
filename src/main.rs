mod creatures;
mod startup;

use bevy::{
    diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin},
    prelude::App,
    DefaultPlugins,
};
use bevy_inspector_egui::quick::WorldInspectorPlugin;

use bevy_sepax2d::prelude::SepaxPlugin;
use creatures::CreaturePlugin;
use startup::StartupPlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(LogDiagnosticsPlugin::default())
        .add_plugin(FrameTimeDiagnosticsPlugin::default())
        .add_plugin(WorldInspectorPlugin)
        .add_plugin(SepaxPlugin)
        .add_plugin(StartupPlugin)
        .add_plugin(CreaturePlugin)
        .run();
}
