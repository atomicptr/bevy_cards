use bevy::{
    ecs::{component::Component, entity::Entity},
    math::Vec2,
    transform::components::Transform,
};

#[derive(Default, Component)]
#[require(Transform)]
pub struct Hoverable;

#[derive(Default, Component)]
#[require(Transform)]
pub struct Hovering;

#[derive(Default, Component)]
#[require(Transform)]
pub struct Draggable;

#[derive(Default, Component)]
#[require(Transform)]
pub struct Dragging;

#[derive(Debug, Default, Component)]
#[require(Transform)]
pub struct CardSize(pub f32, pub f32);

#[derive(Default, Component)]
pub struct AutoZ;

#[derive(Default, Component)]
#[require(Transform, Hoverable, Draggable, AutoZ)]
pub struct Card;

#[derive(Debug, Default, Component)]
pub struct Slot(pub Vec2, pub Option<u16>, pub Option<Entity>);

#[derive(Debug, Default, Component)]
pub struct Slottable(pub Option<u16>);
