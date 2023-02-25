use std::default;

use bevy::{
    prelude::{Assets, Bundle, Color, Commands, Component, Mesh, Plugin, ResMut, Vec2},
    sprite::{ColorMaterial, Material2d, MaterialMesh2dBundle, Sprite, SpriteBundle},
};

const HUNTER_COLOR: Color = Color::rgb(1.0, 0.0, 0.0);
const PREY_COLOR: Color = Color::rgb(0.0, 1.0, 0.0);

#[derive(Component)]
struct CreatureName(String);

#[derive(Bundle)]
struct Creature<T>
where
    T: Material2d,
{
    name: CreatureName,
    mesh: MaterialMesh2dBundle<T>,
}

impl<T> Creature<T>
where
    T: Material2d,
{
    fn new(name: String, mesh: MaterialMesh2dBundle<T>) -> Creature<T> {
        Creature {
            name: CreatureName(name),
            mesh,
        }
    }
}

fn add_hunter(mut commands: Commands, mut meshes: ResMut<Assets<Mesh>>) {
    //    for _ in 1..10 {
    //        commands.spawn();
    //    }
}

fn add_prey(mut commands: Commands) {
    //    for _ in 1..10 {
    //        commands.spawn();
    //    }
}

pub struct EntityPlugin;

impl Plugin for EntityPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_startup_system(add_hunter)
            .add_startup_system(add_prey);
    }
}
