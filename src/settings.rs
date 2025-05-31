use bevy::{ecs::resource::Resource, math::Vec2};

#[derive(Resource)]
pub struct BevyCardsSettings {
    pub card_size: Vec2,
}
