use bevy::{
    ecs::{component::Component, entity::Entity},
    math::Vec2,
    transform::components::Transform,
};

type GroupId = u16;

#[derive(Debug, Component)]
pub struct Card;

#[derive(Component)]
pub struct Draggable;

#[derive(Default, Debug, Component)]
pub struct CardSize(pub Vec2);

impl CardSize {
    pub fn half_size(&self) -> Vec2 {
        self.0 * 0.5
    }
}

#[derive(Default, Debug, Component)]
pub struct Slottable(pub GroupId);

#[derive(Default, Debug, Component)]
pub struct Slot {
    pub size: Vec2,
    pub slot_group: GroupId,
    pub card: Option<Entity>,
}

#[derive(Component)]
pub struct SnapBack;

#[derive(Component)]
pub struct SnapIntoSlot;

#[derive(Default, Component)]
#[require(Transform)]
pub struct Hovering;

#[derive(Default, Component)]
#[require(Transform)]
pub struct Dragging;
