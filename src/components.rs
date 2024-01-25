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
pub struct SolveButton;

#[derive(Component)]
pub struct GenerateButton;

#[derive(Component)]
pub struct CountButton;

#[derive(Component)]
pub struct StatusComponent;