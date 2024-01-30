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
    .add_plugins((DefaultPlugins, bevy_framepace::FramepacePlugin))
    .insert_resource(Playfield::new())
    .insert_resource(Status{text: format!("")})
    .add_systems(Startup, (setup_framelimit, setup))
    .add_systems(Update, field_buttons)
    .add_systems(Update, solve_button)
    .add_systems(Update, generate_button)
    .add_systems(Update, read_values)
    .add_systems(Update, read_status)
    .add_systems(Update, count_button)
    .run();
}

fn setup_framelimit(
    mut settings: ResMut<bevy_framepace::FramepaceSettings>
) {
    use bevy_framepace::Limiter;
    settings.limiter = match settings.limiter {
        Limiter::Auto => Limiter::Off,
        Limiter::Off => Limiter::from_framerate(10.0),
        Limiter::Manual(_) => Limiter::Auto,
    }
}
