use bevy::prelude::*;

#[derive(Component, Clone)]
pub struct Field {
    pub row: usize,
    pub col: usize,
    pub val: Option<u8>,
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

#[derive(Component)]
pub struct Solve;

#[derive(Component)]
pub struct ButtonComponent;

#[derive(Component)]
pub struct Generate;

#[derive(Component)]
pub struct Check;

#[derive(Component)]
pub struct StatusText;

#[derive(Component)]
pub struct CheckText;