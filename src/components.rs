use bevy::{ecs::component::Component, math::Vec2, transform::components::Transform};

#[derive(Default, Component)]
#[require(Transform)]
pub struct CardHoverable;

#[derive(Default, Component)]
#[require(Transform)]
pub struct CardHovering;

#[derive(Default, Component)]
#[require(Transform)]
pub struct CardDraggable;

#[derive(Default, Component)]
#[require(Transform)]
pub struct CardDragging;

#[derive(Debug, Default, Component)]
#[require(Transform)]
pub struct CardSize(pub f32, pub f32);

#[derive(Default, Component)]
pub struct CardAutoZ;

#[derive(Default, Component)]
#[require(Transform, CardHoverable, CardDraggable, CardAutoZ)]
pub struct Card;

#[derive(Debug, Default, Component)]
pub struct CardSlot(pub Vec2, pub Option<u16>);

#[derive(Debug, Default, Component)]
pub struct CardSlottable(pub Option<u16>);
