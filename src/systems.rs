use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts};

use crate::components::*;
use crate::resources::*;
use crate::styles::*;

pub const FIELD_SIZE:f32 = 50.0;

pub fn sidebar(
    mut contexts: EguiContexts,
    mut playfield: ResMut<Playfield>,
) {
    let ctx = contexts.ctx_mut();
    egui::SidePanel::right("side_panel")
        .default_width(200.0)
        .show(ctx, |ui| {
            ui.heading("Settings");
            
            ui.add(egui::Slider::new(&mut playfield.difficulty, 20.0..=60.0.into())
                .fixed_decimals(0)
                .text("Schwierigkeit"));
            
            if ui.button("Neu").clicked() {
                playfield.generate();
            }

            if ui.button("Lösen").clicked() {
                playfield.solve();
            }

            let check_button_text = match playfield.show_errors {
                true => "Fehler verbergen",
                false => "Fehler anzeigen",
            };

            if ui.button(check_button_text).clicked() {
                playfield.show_errors = !playfield.show_errors;
            }
        });
}

pub fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    playfield: Res<Playfield>,
) {
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
        builder.spawn((
            TextBundle::from_section(
                "",
                TextStyle {
                    font: asset_server.load("fonts/FiraSans-Bold.ttf").clone(),
                    font_size: 24.0,
                    color: MENU_BUTTON_COLOR,
                }
            ).with_style(
                Style {
                    grid_column: GridPlacement::span(2),
                    ..default()
                }
            ).with_text_alignment(TextAlignment::Left),
            StatusText
        ));

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
                                        builder.spawn((
                                            ButtonBundle {
                                                style: Style {
                                                    aspect_ratio: Some(1.0),
                                                    width: Val::Px(FIELD_SIZE),
                                                    justify_content: JustifyContent::Center,
                                                    align_items: AlignItems::Center,
                                                    ..default()
                                                },
                                                background_color: BackgroundColor(FIELD_COLOR),
                                                ..default()
                                            },
                                            Field {
                                                row: row,
                                                col: col,
                                                val: Option::None,
                                            }
                                        )).with_children(|builder| {
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
    });
}

pub fn field_button_interaction(
    mut buttons: Query<(&Interaction, &Field), (Changed<Interaction>, With<Field>)>,
    mut playfield: ResMut<Playfield>
) {
    for (interaction, field) in &mut buttons {
        match interaction {
            Interaction::Pressed => {
                if playfield.fixed[(field.row, field.col)] {
                    return;
                }
                let v = (playfield.values[(field.row, field.col)] + 1) % 10;
                playfield.set_value(field.row, field.col, 0);
                if v != 0 {
                    playfield.set_value(field.row, field.col, v);
                }
            },
            Interaction::Hovered => {},
            Interaction::None => {},
        };
    }
}

pub fn field_color(
    mut fields: Query<(&Interaction, &mut BackgroundColor, &Field), With<Field>>,
    playfield: Res<Playfield>
) {
    for (interaction, mut button_color, field) in &mut fields {
        *button_color = match interaction {
            Interaction::Pressed => PRESSED_COLOR,
            Interaction::Hovered => {
                if playfield.fixed[(field.row, field.col)] {
                    FIXED_FIELD_COLOR
                } else {
                    HOVER_COLOR
                }
            },
            Interaction::None => {
                if playfield.fixed[(field.row, field.col)] {
                    FIXED_FIELD_COLOR
                } else {
                    let error = playfield.is_error(field.row, field.col);
                    if error && playfield.show_errors {
                        ERROR_COLOR
                    } else {
                        FIELD_COLOR
                    }
                }
            },
        }.into();
    }
}

pub fn check_button(
    mut buttons: Query<&Interaction, (Changed<Interaction>, With<Check>)>,
    mut playfield: ResMut<Playfield> //, mut status: ResMut<Status>
) {
    if let Ok(interaction) = buttons.get_single_mut() {
        match *interaction {
            Interaction::Pressed => {
                playfield.show_errors = !playfield.show_errors;
                //status.text = format!("{count}");
            }
            Interaction::Hovered => {}
            Interaction::None => {}
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

pub fn read_check_button_status(
    mut buttons: Query<&mut Text, With<CheckText>>,
    playfield: Res<Playfield>
) {
    for mut text  in &mut buttons {
        if playfield.show_errors {
            text.sections[0].value = format!("Fehler verbergen");
        } else {
            text.sections[0].value = format!("Fehler anzeigen");
        }
    }
}

pub fn read_status(
    mut buttons: Query<(&mut Text, With<StatusText>)>,
    playfield: Res<Playfield>
) {
    for (mut text, _ ) in &mut buttons {
        text.sections[0].value = playfield.status_text.clone();
    }
}
