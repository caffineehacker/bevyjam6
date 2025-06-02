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
            .add_systems(OnEnter(GameState::Playing), spawn_everything);

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
struct Creature;

fn spawn_everything(
    mut commands: Commands,
    mut rng: GlobalEntropy<WyRand>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let circle_mesh = meshes.add(Circle::new(2.));
    let uniform = Uniform::new_inclusive(-1000., 1000.);

    for _ in 0..1000 {
        let x = rng.sample(uniform);
        let y = rng.sample(uniform);
        commands.spawn((
            Creature,
            Mesh2d(circle_mesh.clone()),
            MeshMaterial2d(materials.add(Color::LinearRgba(LinearRgba::GREEN))),
            Transform {
                translation: Vec3 { x, y, z: 0. },
                ..default()
            },
        ));
    }
}
