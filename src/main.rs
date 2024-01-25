pub mod components;
pub mod resources;
pub mod styles;
mod systems;

use array2d::Array2D;
use bevy::prelude::*;
use rand::prelude::*;
use systems::*;
use resources::{Playfield, Status};

pub const FIELD_SIZE:f32 = 50.0;

fn main() {
    App::new()
    .add_plugins(DefaultPlugins)
    .insert_resource(Playfield::new())
    .insert_resource(Status{text: format!("x")})
    .add_systems(Startup, setup)
    .add_systems(Update, field_buttons)
    .add_systems(Update, solve_button)
    .add_systems(Update, generate_button)
    .add_systems(Update, read_values)
    .add_systems(Update, read_status)
    .add_systems(Update, count_button)
    .run();
}
