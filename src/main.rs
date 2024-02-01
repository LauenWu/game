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
    .add_plugins((DefaultPlugins, bevy_framepace::FramepacePlugin))
    .insert_resource(Playfield::new())
    .add_systems(Startup, (setup_framelimit, setup))
    .add_systems(Update, field_button_interaction)
    .add_systems(Update, (field_color, button_interaction))
    .add_systems(Update, solve_button)
    .add_systems(Update, generate_button)
    .add_systems(Update, read_values)
    .add_systems(Update, read_status)
    .add_systems(Update, check_button)
    .add_systems(Update, read_check_button_status)
    .run();
}

fn setup_framelimit(
    mut settings: ResMut<bevy_framepace::FramepaceSettings>
) {
    use bevy_framepace::Limiter;
    settings.limiter = Limiter::from_framerate(10.0)
}
