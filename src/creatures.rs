use bevy::ecs::change_detection::Mut;
use bevy::prelude::{Input, IntoSystemDescriptor, KeyCode, Query, Res, SystemSet, With};
use bevy::time::Time;
use bevy::{
    prelude::{
        default, shape, Assets, BuildChildren, Bundle, Color, Commands, Component, Handle, Mesh,
        Plugin, ResMut, Transform, Vec2, Vec3,
    },
    sprite::{ColorMaterial, Material2d, MaterialMesh2dBundle, Mesh2dHandle, Sprite, SpriteBundle},
};

use bevy_sepax2d::{
    prelude::{Movable, Sepax},
    Convex,
};
use rand::random;
use sepax2d::prelude::Circle as SepaxCircle;

const HUNTER_COLOR: Color = Color::rgb(1.0, 0.0, 0.0);
const PREY_COLOR: Color = Color::rgb(0.0, 0.0, 1.0);
const CREATURE_COUNT: u64 = 1;
const CREATURE_SIZE: f32 = 30.0;

const FIELD_SIZE: f32 = 600.0;

const PREY_ENERGY_REDUCTION_RATE: f32 = 0.2;
const PREY_ENERGY_PROPAGATION_RATE: f32 = 0.2;

const PREY_REPRODUCTION_RATE: f32 = 0.1;
const HUNTER_REPRODUCTION_RATE: f32 = 0.5;

#[derive(Component)]
struct CreatureName(String);

#[derive(Component)]
pub struct Hunter;

#[derive(Component)]
pub struct Prey;

#[derive(Component, Clone, Copy)]
pub struct Creature {
    pub speed: f32,
    pub angular_speed: f32,
    pub energy: f32,
    pub reproductivity: f32,
}

impl Creature {
    fn new() -> Creature {
        Creature {
            speed: 0.0,
            angular_speed: 0.0,
            energy: 1.0,
            reproductivity: 0.0,
        }
    }

    pub fn set_speed(&mut self, speed: f32, angular_speed: f32) {
        self.speed = speed;
        self.angular_speed = angular_speed;
    }

    pub fn perform_move(&mut self, delta_t: f32, mut transform: Mut<Transform>) {
        let d_distance = self.speed * delta_t;
        let d_angle = self.angular_speed * delta_t;

        transform.rotate_local_z(d_angle);

        if (self.energy > 0.0) {
            let v_distance = transform.local_y() * d_distance;
            transform.translation += v_distance
        }
    }

    pub fn reproduce(&mut self) -> Creature {
        self.clone()
    }

    pub fn set_energy(&mut self, mut energy: f32) {
        if energy > 1.0 {
            energy = 1.0
        } else if energy < 0.0 {
            energy = 0.0
        }

        self.energy = energy
    }
}

#[derive(Bundle)]
struct DirectionIndicator {
    sprite: SpriteBundle,
}

impl DirectionIndicator {
    fn new(color: Color) -> DirectionIndicator {
        DirectionIndicator {
            sprite: SpriteBundle {
                sprite: Sprite {
                    color,
                    custom_size: Some(Vec2::new(0.2, 0.8)),
                    ..default()
                },
                transform: Transform::from_translation(Vec3::new(0.0, 0.5, 0.0)),
                ..default()
            },
        }
    }
}

#[derive(Bundle)]
pub struct CreatureBundle<T>
where
    T: Material2d,
{
    name: CreatureName,
    creature: Creature,
    mesh: MaterialMesh2dBundle<T>,
    collision: Sepax,
    movable: Movable,
}

impl<T> CreatureBundle<T>
where
    T: Material2d,
{
    fn new(
        name: String,
        mesh: Mesh2dHandle,
        material: Handle<T>,
        field_size: f32,
        size: f32,
    ) -> CreatureBundle<T> {
        let position = random_position(field_size);

        CreatureBundle {
            name: CreatureName(name),
            mesh: MaterialMesh2dBundle {
                mesh: mesh,
                material: material,
                transform: Transform::from_translation(position).with_scale(Vec3 {
                    x: size,
                    y: size,
                    z: 0.0,
                }),
                ..default()
            },
            collision: Sepax {
                convex: Convex::Circle(SepaxCircle::new((position.x, position.y), size)),
            },
            movable: Movable {
                axes: vec![(0.0, 0.0)],
            },
            creature: Creature::new(),
        }
    }
}

fn random_position(field_size: f32) -> Vec3 {
    let x: f32 = random::<f32>() * field_size - field_size / 2.0;
    let y: f32 = random::<f32>() * field_size - field_size / 2.0;

    return Vec3::new(x, y, 0.0);
}

fn add_hunter(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    for _ in 0..CREATURE_COUNT {
        commands
            .spawn((
                CreatureBundle::new(
                    "hunter_{n}".to_string(),
                    meshes.add(shape::Circle::default().into()).into(),
                    materials.add(ColorMaterial::from(HUNTER_COLOR)),
                    FIELD_SIZE,
                    CREATURE_SIZE,
                ),
                Hunter,
            ))
            .with_children(|parent| {
                parent.spawn(DirectionIndicator::new(HUNTER_COLOR));
            });
    }
}

fn add_prey(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    for _ in 0..CREATURE_COUNT {
        commands
            .spawn((
                CreatureBundle::new(
                    "prey_{n}".to_string(),
                    meshes.add(shape::Circle::default().into()).into(),
                    materials.add(ColorMaterial::from(PREY_COLOR)),
                    FIELD_SIZE,
                    CREATURE_SIZE,
                ),
                Prey,
            ))
            .with_children(|parent| {
                parent.spawn(DirectionIndicator::new(PREY_COLOR));
            });
    }
}

fn set_creature_movement(
    keys: Res<Input<KeyCode>>,
    mut creature_query: Query<&mut Creature, With<Prey>>,
) {
    let mut creature = creature_query.single_mut();

    let mut speed = creature.speed;
    let mut angular_speed = creature.angular_speed;

    if keys.just_pressed(KeyCode::W) {
        speed = 50.0;
    }
    if keys.just_released(KeyCode::W) {
        speed = 0.0;
    }

    if keys.just_pressed(KeyCode::A) {
        angular_speed = 2.0;
    }
    if keys.just_released(KeyCode::A) {
        angular_speed = 0.0;
    }

    if keys.just_pressed(KeyCode::D) {
        angular_speed = -2.0;
    }
    if keys.just_released(KeyCode::D) {
        angular_speed = 0.0;
    }

    creature.set_speed(speed, angular_speed);
}

fn move_creature(
    mut creature_query: Query<(&mut Creature, &mut Transform), With<Prey>>,
    time: Res<Time>,
) {
    let (mut creature, transform) = creature_query.single_mut();

    let delta_t = time.delta().as_secs_f32();

    creature.perform_move(delta_t, transform);
}

fn update_energy(
    //    mut hunter_query: Query<&mut Creature, With<Hunter>>,
    mut prey_query: Query<&mut Creature, With<Prey>>,
    time: Res<Time>,
) {
    let delta_t = time.delta().as_secs_f32();

    for mut creature in prey_query.iter_mut() {
        let energy;

        if creature.speed == 0.0 {
            energy = creature.energy + PREY_ENERGY_PROPAGATION_RATE * delta_t;
        } else {
            energy = creature.energy - PREY_ENERGY_REDUCTION_RATE * delta_t;
        }

        creature.set_energy(energy)
    }
}

pub struct CreaturePlugin;

impl Plugin for CreaturePlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_startup_system(add_hunter)
            .add_startup_system(add_prey)
            .add_system_set(
                SystemSet::new()
                    .with_system(set_creature_movement)
                    .with_system(move_creature.after(set_creature_movement))
                    .with_system(update_energy.after(set_creature_movement)),
            );
    }
}
