#![allow(clippy::type_complexity)]

use crate::menu::MenuPlugin;

use bevy::app::App;
#[cfg(debug_assertions)]
use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};
use bevy::prelude::*;
use bevy_rand::prelude::*;
use rand::{Rng, distributions::Uniform};

// This example game uses States to separate logic
// See https://bevy-cheatbook.github.io/programming/states.html
// Or https://github.com/bevyengine/bevy/blob/main/examples/ecs/state.rs
#[derive(States, Default, Clone, Eq, PartialEq, Debug, Hash)]
pub enum GameState {
    // During this State the actual game logic is executed
    Playing,
    // Here the menu is drawn and waiting for player interaction
    #[default]
    Menu,
}

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<GameState>()
            .add_plugins(MenuPlugin)
            .add_systems(OnEnter(GameState::Playing), spawn_everything)
            .add_systems(
                FixedUpdate,
                (move_random, adjust_health, update_count)
                    .chain()
                    .run_if(in_state(GameState::Playing)),
            )
            .insert_resource(HealthColors(Vec::new()));

        #[cfg(debug_assertions)]
        {
            app.add_plugins((
                FrameTimeDiagnosticsPlugin::default(),
                LogDiagnosticsPlugin::default(),
            ));
        }
    }
}

#[derive(Component)]
struct Creature {
    health: i32,
}

impl Default for Creature {
    fn default() -> Self {
        Self { health: 100 }
    }
}

#[derive(Resource)]
struct HealthColors(Vec<Handle<ColorMaterial>>);

#[derive(Component)]
struct CellCountText;

fn spawn_everything(
    mut commands: Commands,
    mut rng: GlobalEntropy<WyRand>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut health_colors: ResMut<HealthColors>,
) {
    for i in 0..10 {
        health_colors
            .0
            .push(materials.add(Color::LinearRgba(LinearRgba::new(
                1. - (i as f32 / 9.),
                i as f32 / 9.,
                0.,
                1.,
            ))));
    }
    let circle_mesh = meshes.add(Circle::new(2.));
    let uniform = Uniform::new_inclusive(-1000., 1000.);

    for _ in 0..1000 {
        let x = rng.sample(uniform);
        let y = rng.sample(uniform);
        commands.spawn((
            Creature::default(),
            Mesh2d(circle_mesh.clone()),
            MeshMaterial2d(health_colors.0.get(9).unwrap().clone()),
            Transform {
                translation: Vec3 { x, y, z: 0. },
                ..default()
            },
            rng.fork_rng(),
        ));
    }

    commands.spawn(Text::new("Cells: ")).with_child((
        CellCountText,
        TextSpan::default(),
        TextFont {
            font_size: 20.0,
            ..default()
        },
        TextColor(Color::linear_rgb(0.9, 0.9, 0.9)),
    ));
}

fn move_random(mut q: Query<(&mut Transform, &mut Entropy<WyRand>), With<Creature>>) {
    q.par_iter_mut().for_each(|(mut t, mut rng)| {
        let move_distance = rng.gen_range(0.0..=5.0);
        let rotation = Rot2::degrees(rng.gen_range(-180.0..=180.0));
        t.translation.x += rotation.cos * move_distance;
        t.translation.y += rotation.sin * move_distance;
    });
}

fn adjust_health(
    commands: ParallelCommands,
    mut q: Query<(
        &mut Entropy<WyRand>,
        &mut Creature,
        &mut MeshMaterial2d<ColorMaterial>,
        Entity,
    )>,
    health_colors: Res<HealthColors>,
) {
    q.par_iter_mut()
        .for_each(|(mut rng, mut creature, mut material, entity)| {
            let should_decrease = rng.gen_bool(0.1);
            if should_decrease {
                creature.health -= 1;
                if creature.health <= 0 {
                    commands.command_scope(|mut commands| commands.entity(entity).despawn());
                }
                material.0 = health_colors
                    .0
                    .get(creature.health as usize / 10)
                    .unwrap()
                    .clone();
            }
        });
}

fn update_count(
    mut q: Single<&mut TextSpan, With<CellCountText>>,
    creatures: Query<Entity, With<Creature>>,
) {
    q.0 = creatures.iter().count().to_string();
}
