use bevy::prelude::*;

use crate::components::*;
use crate::resources::*;
use crate::styles::*;

pub const FIELD_SIZE:f32 = 50.0;

pub fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    playfield: Res<Playfield>,
) {
    println!("s2");
    commands.spawn(Camera2dBundle::default());

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
            background_color: BackgroundColor(BACKGROUND_COLOR),
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
                flex_direction: FlexDirection::Column,
                //justify_content: JustifyContent::Center,
                align_items: AlignItems::Start,
                height: Val::Percent(100.0),
                padding: UiRect::all(Val::Px(4.0)),
                ..default()
            },
            ..default()
        }).with_children(|builder| {
            builder.spawn((
                ButtonBundle {
                    style: Style {
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        width: Val::Percent(100.0),
                        margin: UiRect::all(Val::Px(4.0)),
                        height: Val::Px(35.0),
                        ..default()
                    },
                    background_color: BackgroundColor(MENU_BUTTON_COLOR),
                    ..default()
                },
                SolveButton
            ))
            .with_children(|builder| {
                builder.spawn(TextBundle {
                    text: Text {
                        sections: vec![
                            TextSection::new(
                                "solve",
                                get_text_style(&asset_server).clone()
                            )
                        ],
                        alignment: TextAlignment::Center,
                        ..default()
                    },
                    ..default()
                });
            });

            builder.spawn((
                ButtonBundle {
                    style: Style {
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        width: Val::Percent(100.0),
                        margin: UiRect::all(Val::Px(4.0)),
                        height: Val::Px(35.0),
                        ..default()
                    },
                    background_color: BackgroundColor(MENU_BUTTON_COLOR),
                    ..default()
                },
                GenerateButton
            ))
            .with_children(|builder| {
                builder.spawn(TextBundle {
                    text: Text {
                        sections: vec![
                            TextSection::new(
                                "generate",
                                get_text_style(&asset_server).clone()
                            )
                        ],
                        alignment: TextAlignment::Center,
                        ..default()
                    },
                    ..default()
                });
            });
        });
    });
}

pub fn field_buttons(
    mut buttons: Query<(&Interaction, &mut BackgroundColor, &Field), (Changed<Interaction>, With<Field>)>,
    mut playfield: ResMut<Playfield>
) {
    for (interaction, mut button_color, field) in &mut buttons {
        *button_color = match interaction {
            Interaction::Pressed => {
                let v = playfield.values[(field.row, field.col)];
                playfield.values[(field.row, field.col)] = (v + 1)%10;
                PRESSED_COLOR
            },
            Interaction::Hovered => HOVER_COLOR,
            Interaction::None => FIELD_COLOR,
        }.into();
    }
}

pub fn generate_button(
    mut buttons: Query<(&Interaction, &mut BackgroundColor), (Changed<Interaction>, With<GenerateButton>)>
) {
    if let Ok((interaction, mut backgroud_color)) = buttons.get_single_mut() {
        match *interaction {
            Interaction::Pressed => {
                println!("generate");
            }
            Interaction::Hovered => {
               *backgroud_color = HOVER_COLOR.into();
            }
            Interaction::None => {
                *backgroud_color = MENU_BUTTON_COLOR.into();
             }
        }
    }
}

pub fn solve_button(
    mut buttons: Query<(&Interaction, &mut BackgroundColor), (Changed<Interaction>, With<SolveButton>)>,
    mut playfield: ResMut<Playfield>
) {
    if let Ok((interaction, mut backgroud_color)) = buttons.get_single_mut() {
        match *interaction {
            Interaction::Pressed => {
                println!("solve");
                solve(&mut playfield);
            }
            Interaction::Hovered => {
               *backgroud_color = HOVER_COLOR.into();
            }
            Interaction::None => {
                *backgroud_color = MENU_BUTTON_COLOR.into();
             }
        }
    }
}

pub fn read_values(
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

// enum SolvableState {
//     Specifying, // when conditions for generation are not yet defined
//     Generating, // when conditions for generation are defined, but conditions for solving not
//     Solving, // when conditions for solving are defined 
//     Solved, // when all fields are set and all conditions are met    
// }