use bevy::{
    ecs::{entity::Entity, event::Event},
    math::Vec2,
};

#[derive(Event)]
pub struct DraggingStartedEvent(pub Entity, pub Vec2);

#[derive(Event)]
pub struct DraggingStoppedEvent(pub Entity, pub Vec2);
