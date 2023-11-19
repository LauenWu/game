use bevy::{
    prelude::*,
    window::PrimaryWindow,
    ecs::system::EntityCommands,
    text::{BreakLineOn, Text2dBounds, TextLayoutInfo}
};
use bevy_mod_picking::prelude::*;
use array2d::Array2D;


pub const FIELD_SIZE:f32 = 50.0;
pub const ROW_COUNT:usize = 9;
pub const COL_COUNT:usize = 9;

pub const FIELD_COLOR:Color = Color::WHITE;
pub const FIXED_FIELD_COLOR:Color = Color::rgb(0.75, 0.75, 0.75);
pub const HOVER_COLOR:Color = Color::rgb(0.35, 0.75, 0.35);
pub const PRESSED_COLOR:Color = Color::DARK_GRAY;

pub const VAL_COLOR:Color = Color::DARK_GRAY;
pub const FIXED_VAL_COLOR:Color = Color::BLACK;
pub const TEXT_COLOR:Color = Color::BLACK;

// pub static mut playfield:[[u8;9];9] = [
//     [0,0,0,1,1,1,2,2,2,],
//     [0,0,0,1,1,1,2,2,2,],
//     [0,0,0,1,1,1,2,2,2,],
//     [3,3,3,4,4,4,5,5,5,],
//     [3,3,3,4,4,4,5,5,5,],
//     [3,3,3,4,4,4,5,5,5,],
//     [6,6,6,7,7,7,8,8,8,],
//     [6,6,6,7,7,7,8,8,8,],
//     [6,6,6,7,7,7,8,8,8,],
// ];

fn main() {
    App::new()
    .add_plugins(DefaultPlugins)
    .add_plugins(DefaultPickingPlugins.build().disable::<DebugPickingPlugin>())
    // uncomment the next line to get the debug cursor tooltips
    //.add_plugins(DefaultPickingPlugins)
    .add_systems(Startup, setup)
    .add_systems(Update, button_interactions)
    .add_systems(Update, read_values)
    .run();
}

pub fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    println!("s2");
    commands.spawn(Camera2dBundle::default());

    let playfield = Playfield {
        values: Array2D::filled_with(0,ROW_COUNT,COL_COUNT),
        selected_field: None
    };
    commands.insert_resource(playfield.clone());

    commands.spawn(
        NodeBundle {
            style: Style {
                display: Display::Grid,
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                align_items: AlignItems::Center,
                grid_template_columns: vec![GridTrack::flex(1.0), GridTrack::flex(0.2)],
                grid_template_rows: vec![
                    GridTrack::px(25.),
                    GridTrack::flex(1.0),
                    GridTrack::px(25.),
                ],
                ..default()
            },
            background_color: BackgroundColor(Color::DARK_GRAY),
            ..default()
        }
    ).with_children(|builder| {
        // Header
        builder.spawn(TextBundle::from_section(
            "",
            get_text_style(&asset_server)
        ).with_style(
            Style {
                grid_column: GridPlacement::span(2),
                ..default()
            }
        ).with_text_alignment(TextAlignment::Left));

        // Main content
        builder.spawn((
            // left container
            NodeBundle {
                style: Style {
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    height: Val::Percent(100.0),
                    ..default()
                },
                ..default()
            },
        )).with_children(|builder| {
            builder.spawn((
                NodeBundle {
                    style: Style {
                        display: Display::Grid,
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        aspect_ratio: Some(1.0),
                        grid_template_columns: RepeatedGridTrack::flex(3, 1.0),
                        grid_template_rows: RepeatedGridTrack::flex(3, 1.0),
                        row_gap: Val::Px(0.0),
                        column_gap: Val::Px(0.0),
                        ..default()
                    },
                    ..default()
                },
            )).with_children(|builder| {
                for quad_row in 0..3 {
                    for quad_col in 0..3 {
                        builder.spawn((
                            NodeBundle {
                                style: Style {
                                    display: Display::Grid,
                                    justify_content: JustifyContent::Center,
                                    align_items: AlignItems::Center,
                                    aspect_ratio: Some(1.0),
                                    grid_template_columns: RepeatedGridTrack::flex(3, 1.0),
                                    grid_template_rows: RepeatedGridTrack::flex(3, 1.0),
                                    row_gap: Val::Px(0.0),
                                    column_gap: Val::Px(0.0),
                                    padding: UiRect::all(Val::Px(2.0)),
                                    ..default()
                                },
                                ..default()
                            },
                        )).with_children(|builder| {
                            for field_row in 0..3 {
                                for field_col in 0..3 {
                                    let row = quad_row * 3 + field_row;
                                    let col = quad_col * 3 + field_col;
                                    let fixed = playfield.values[(row, col)] > 0;

                                    builder.spawn((
                                        NodeBundle {
                                            style: Style {
                                                display: Display::Grid,
                                                padding: UiRect::all(Val::Px(1.0)),
                                                ..default()
                                            },
                                            ..default()
                                        },
                                    ))
                                    .with_children(|builder| {
                                        let button_bundle = ButtonBundle {
                                            style: Style {
                                                aspect_ratio: Some(1.0),
                                                width: Val::Px(FIELD_SIZE),
                                                justify_content: JustifyContent::Center,
                                                align_items: AlignItems::Center,
                                                ..default()
                                            },
                                            background_color: BackgroundColor(FIELD_COLOR),
                                            ..default()
                                        };
                                        let mut parent;
                                        if playfield.values[(row, col)] > 0 {
                                            parent = builder.spawn((
                                                button_bundle,
                                                FixedField {
                                                    row: row,
                                                    col: col,
                                                }
                                            ));
                                        } else {
                                            parent = builder.spawn((
                                                button_bundle,
                                                Field {
                                                    row: row,
                                                    col: col,
                                                    val: Option::None,
                                                    pressed: false,
                                                }
                                            ));
                                        };
                                        parent.with_children(|builder| {
                                            builder.spawn((
                                                TextBundle {
                                                    text: Text {
                                                        sections: vec![
                                                            TextSection::new(
                                                                "",
                                                                get_field_text_style(&asset_server, fixed).clone()
                                                            )
                                                        ],
                                                        alignment: TextAlignment::Center,
                                                        ..default()
                                                    },
                                                    ..default()
                                                },
                                                Value {
                                                    row: row,
                                                    col: col,
                                                },
                                            ));
                                        });
                                    });
                                }
                            }
                        });
                    }
                }
            });
        });

        // right container
        builder.spawn(NodeBundle {
            style: Style {
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                height: Val::Percent(100.0),
                ..default()
            },
            ..default()
        });
    });
}

fn button_interactions(
    mut buttons: Query<(&Interaction, &mut BackgroundColor, &mut Field), With<Button>>,
    mut playfield: ResMut<Playfield>
) {
    for (interaction, mut button_color, mut field) in &mut buttons {
        *button_color = match interaction {
            Interaction::Pressed => {
                if !field.pressed {
                    field.pressed = true;
                    let v = playfield.values[(field.row, field.col)];
                    playfield.values[(field.row, field.col)] = (v + 1)%10;
                }               

                PRESSED_COLOR
            },
            Interaction::Hovered => {
                field.pressed = false;
                HOVER_COLOR
            },
            Interaction::None => {
                field.pressed = false;
                FIELD_COLOR
            },
        }.into();
    }
}

fn read_values(
    mut buttons: Query<(&mut Text, &Value)>,
    playfield: Res<Playfield>
) {
    for (mut text, value ) in &mut buttons {
        let v = playfield.values[(value.row, value.col)];

        if v > 0 {
            text.sections[0].value = format!("{v}");
        } else {
            text.sections[0].value = format!("");
        }
    }
}

fn get_field_text_style(asset_server: &Res<AssetServer>, fixed: bool) -> TextStyle {
    let font = asset_server.load("fonts/FiraSans-Bold.ttf");
    TextStyle {
        font: font.clone(),
        font_size: 38.0,
        color: if fixed {FIXED_VAL_COLOR} else {VAL_COLOR},
    }
}

fn get_text_style(asset_server: &Res<AssetServer>) -> TextStyle {
    let font = asset_server.load("fonts/FiraSans-Bold.ttf");
    TextStyle {
        font: font.clone(),
        font_size: 20.0,
        color: TEXT_COLOR,
    }
}

#[derive(Resource, Clone)]
pub struct Playfield {
    pub values: Array2D<u8>,
    pub selected_field: Option<Field>,
}

#[derive(Component, Clone)]
pub struct Field {
    pub row: usize,
    pub col: usize,
    pub val: Option<u8>,
    pub pressed: bool,
}

#[derive(Component)]
pub struct FixedField {
    pub row: usize,
    pub col: usize,
}

#[derive(Component)]
pub struct Value {
    pub row: usize,
    pub col: usize,
}