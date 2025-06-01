use bevy::{
    ecs::{entity::Entity, event::Event},
    math::Vec2,
};

#[derive(Event)]
pub struct DragStartedEvent(pub Entity, pub Vec2);

#[derive(Event)]
pub struct DragEndedEvent(pub Entity, pub Vec2);

#[derive(Event)]
pub struct HoverStartedEvent(pub Entity);

#[derive(Event)]
pub struct HoverEndedEvent(pub Entity);

#[derive(Event)]
pub struct SlottedIntoEvent {
    pub card: Entity,
    pub slot: Entity,
}

#[derive(Event)]
pub struct UnknownSlotTargetEvent(pub Entity);
