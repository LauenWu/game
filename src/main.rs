pub mod components;
pub mod resources;
pub mod styles;
mod systems;

use array2d::Array2D;
use bevy::prelude::*;
use rand::prelude::*;
use systems::*;
use resources::Playfield;

pub const FIELD_SIZE:f32 = 50.0;

fn main() {
    App::new()
    .add_plugins(DefaultPlugins)
    .insert_resource(Playfield::new())
    .add_systems(Startup, setup)
    .add_systems(Update, field_buttons)
    .add_systems(Update, solve_button)
    .add_systems(Update, generate_button)
    .add_systems(Update, read_values)
    .run();
}
