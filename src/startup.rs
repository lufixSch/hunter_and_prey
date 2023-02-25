use bevy::prelude::{App, Camera2dBundle, Commands, Plugin};

fn startup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

pub struct StartupPlugin;

impl Plugin for StartupPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(startup);
    }
}
