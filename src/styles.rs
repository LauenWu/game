use bevy::prelude::*;

pub const FIELD_COLOR:Color = Color::WHITE;
pub const FIXED_FIELD_COLOR:Color = Color::rgb(0.8, 0.8, 0.8);
pub const HOVER_COLOR:Color = Color::rgb(0.4, 0.8, 0.4);
pub const PRESSED_COLOR:Color = Color::rgb(0.5, 0.9, 0.5);

pub const TEXT_COLOR:Color = Color::rgb(0.25, 0.25, 0.25);
pub const FIXED_TEXT_COLOR:Color = Color::BLACK;

pub const BACKGROUND_COLOR:Color = Color::rgb(192./255., 2./255., 244./255.);
pub const MENU_BUTTON_COLOR:Color = Color::WHITE;
pub const MENU_BUTTON_TEXT_COLOR:Color = Color::rgb(0.2, 0.2, 0.2);

pub fn get_field_text_style(asset_server: &Res<AssetServer>, fixed: bool) -> TextStyle {
    let font = asset_server.load("fonts/FiraSans-Bold.ttf");
    TextStyle {
        font: font.clone(),
        font_size: 42.0,
        color: if fixed {FIXED_TEXT_COLOR} else {TEXT_COLOR},
    }
}

pub fn get_text_style(asset_server: &Res<AssetServer>) -> TextStyle {
    let font = asset_server.load("fonts/FiraSans-Bold.ttf");
    TextStyle {
        font: font.clone(),
        font_size: 24.0,
        color: TEXT_COLOR,
    }
}