use bevy::{
    prelude::*,
    window::PrimaryWindow,
    ecs::system::EntityCommands,
    text::{BreakLineOn, Text2dBounds, TextLayoutInfo}
};
use bevy_mod_picking::prelude::*;


pub const FIELD_SIZE:f32 = 50.0;
pub const ROW_COUNT:u16 = 9;
pub const COL_COUNT:u16 = 9;

fn main() {
    App::new()
    .add_plugins(DefaultPlugins)
    .add_plugins(DefaultPickingPlugins.build().disable::<DebugPickingPlugin>())
    // uncomment the next line to get the debug cursor tooltips
    //.add_plugins(DefaultPickingPlugins)
    .add_systems(Startup, setup)
    .add_systems(Update, update_button_colors)
    .run();
}

pub fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    commands.spawn(Camera2dBundle::default());

    let font = asset_server.load("fonts/FiraSans-Bold.ttf");
    commands.spawn(
        NodeBundle {
            style: Style {
                display: Display::Grid,
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                align_items: AlignItems::Center,
                grid_template_columns: vec![GridTrack::flex(1.0), GridTrack::flex(0.2)],
                grid_template_rows: vec![
                    GridTrack::px(20.),
                    GridTrack::flex(1.0),
                    GridTrack::px(20.),
                ],
                ..default()
            },
            background_color: BackgroundColor(Color::WHITE),
            ..default()
        }
    ).with_children(|builder| {
        // Header
        builder.spawn(TextBundle::from_section(
            "test",
            TextStyle {
                font: font.clone(),
                font_size: 18.0,
                color: Color::GRAY,
            }
        ).with_style(
            Style {
                grid_column: GridPlacement::span(2),
                ..default()
            }
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
                background_color: BackgroundColor(Color::DARK_GRAY),
                ..default()
            },
            //Pickable::IGNORE
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
                    background_color: BackgroundColor(Color::DARK_GRAY),
                    ..default()
                },
                //Pickable::IGNORE
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
                                background_color: BackgroundColor(Color::DARK_GRAY),
                                ..default()
                            },
                            //Pickable::IGNORE
                        )).with_children(|builder| {
                            for field_row in 0..3 {
                                for field_col in 0..3 {
                                    let row = quad_row * 3 + field_row;
                                    let col = quad_col * 3 + field_col;

                                    builder.spawn((
                                        NodeBundle {
                                            style: Style {
                                                display: Display::Grid,
                                                padding: UiRect::all(Val::Px(1.0)),
                                                ..default()
                                            },
                                            background_color: BackgroundColor(Color::DARK_GRAY),
                                            ..default()
                                        },
                                        //Pickable::IGNORE
                                    ))
                                    .with_children(|builder| {
                                        builder.spawn((
                                            ButtonBundle {
                                                style: Style {
                                                    display: Display::Grid,
                                                    aspect_ratio: Some(1.0),
                                                    width: Val::Px(FIELD_SIZE),
                                                    ..default()
                                                },
                                                background_color: BackgroundColor(Color::WHITE),
                                                ..default()
                                            },
                                            Field {
                                                row: row,
                                                col: col
                                            },
                                            On::<Pointer<Click>>::run(move || info!("Button pressed!")),
                                            // Buttons should not deselect other things:
                                            NoDeselect,
                                        ));
                                        // .with_children(|builder| {
                                        //     builder.spawn((
                                        //         TextBundle {
                                        //             text: Text {
                                        //                 sections: vec![
                                        //                     TextSection::new(
                                        //                         format!("{row},{col}"),
                                        //                         style.clone()
                                        //                     )
                                        //                 ],
                                        //                 alignment: TextAlignment::Center,
                                        //                 ..default()
                                        //             },
                                        //             ..default()
                                        //         },
                                        //         Value
                                        //     ));
                                        // });
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
            background_color: BackgroundColor(Color::BLACK),
            ..default()
        });
    });
}

/// Use the [`PickingInteraction`] state of each button to update its color.
fn update_button_colors(
    mut buttons: Query<(Option<&PickingInteraction>, &mut BackgroundColor), With<Button>>,
) {
    for (interaction, mut button_color) in &mut buttons {
        *button_color = match interaction {
            Some(PickingInteraction::Pressed) => Color::rgb(0.35, 0.75, 0.35),
            Some(PickingInteraction::Hovered) => Color::rgb(0.75, 0.75, 0.75),
            Some(PickingInteraction::None) | None => Color::WHITE,
        }
        .into();
    }
}

pub fn hover(
    mut button_query: Query<&mut Style, With<Field>>
) {
    for mut button in &mut button_query {
        
        println!("{:?}", button);
    }
}

/* pub fn hover(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    q_parent: Query<&Field>,
    q_child: Query<(&Parent, (Entity, With<Text>))>,
) {
    let window = window_query.single();

    let Some(position) = window.cursor_position() else { return };
    // println!("Cursor is inside the primary window, at {:?}", position);
    // let x = window.width()/2.0 - 226.0;
    // let y = window.height()/2.0 - 226.0;
    let x = position.x;
    let y = position.y;

    
    for (parent, (text_entity, _)) in q_child.iter() {
        let field = q_parent.get(parent.get()).unwrap();

        let row = field.row;
        let col = field.col;

        let x_min = x + ((col*50) as f32);
        let y_min = y + ((row*50) as f32);
        
        if position.x > x_min && position.x < x_min + 50.0 && position.y > y_min && position.y < y_min + 50.0 {
            println!("hover {:?}", position);
            let mut field_e = commands.entity(parent.get());
            field_e.remove_children(&[text_entity]);
            
            // field_e.add_child(
            //     commands.spawn(
            //         Text {
            //             sections: vec![TextSection::new(
            //                 "",
            //                 font.style.clone(),
            //             )],
            //             alignment: TextAlignment::Center,
            //             linebreak_behavior: BreakLineOn::NoWrap,
            //         }
            //     ).id()
            // );
        }
    }
} */

/* pub fn print_fields(query: Query<&Field>) {
    for field in query.iter() {
        println!("x:{}, y:{}, v:{}", field.row, field.col, field.val)
    }
} */

#[derive(Resource)]
pub struct Font {
    pub style:TextStyle,
}

#[derive(Component, Default, Debug)]
pub struct Field {
    pub row: usize,
    pub col: usize,
}

#[derive(Component)]
pub struct Value;

// #[derive(Bundle, Clone, Default)]
// pub struct Text2dBundle {
//     /// Contains the text.
//     pub text: Text,
//     /// How the text is positioned relative to its transform.
//     pub text_anchor: Anchor,
//     /// The maximum width and height of the text.
//     pub text_2d_bounds: Text2dBounds,
//     /// The transform of the text.
//     pub transform: Transform,
//     /// The global transform of the text.
//     pub global_transform: GlobalTransform,
//     /// The visibility properties of the text.
//     pub visibility: Visibility,
//     /// Inherited visibility of an entity.
//     pub inherited_visibility: InheritedVisibility,
//     /// Algorithmically-computed indication of whether an entity is visible and should be extracted for rendering
//     pub view_visibility: ViewVisibility,
//     /// Contains the size of the text and its glyph's position and scale data. Generated via [`TextPipeline::queue_text`]
//     pub text_layout_info: TextLayoutInfo,
//     pub sprite: SpriteBundle,
// }

// #[derive(Bundle, Clone, Default)]
// pub struct SpriteBundle {
//     pub sprite: Sprite,
//     pub transform: Transform,
//     pub global_transform: GlobalTransform,
//     pub texture: Handle<Image>,
//     /// User indication of whether an entity is visible
//     pub visibility: Visibility,
//     /// Inherited visibility of an entity.
//     pub inherited_visibility: InheritedVisibility,
//     /// Algorithmically-computed indication of whether an entity is visible and should be extracted for rendering
//     pub view_visibility: ViewVisibility,
//     pub text: Text2dBundle,
// }

#[derive(Bundle, Default)]
struct FieldBundle {
    pub field: Field,
    pub sprite: SpriteBundle,
    //pub text: Text,
    // pub transform: Transform,
    // pub global_transform: GlobalTransform,
    // pub visibility: Visibility,
    // pub inherited_visibility: InheritedVisibility,
    // pub view_visibility: ViewVisibility,
}