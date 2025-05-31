use bevy::{
    app::{App, Plugin, Update},
    math::Vec2,
};

use crate::{
    behaviour::{LastAutoZ, Pointer, draggable, dragging, hoverable, update_pointer},
    events::{DraggingStartedEvent, DraggingStoppedEvent},
    settings::BevyCardsSettings,
};

pub struct BevyCardsPlugin {
    pub card_size: Vec2,
    pub initial_auto_z_value: Option<f32>,
}

impl Plugin for BevyCardsPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(BevyCardsSettings {
            card_size: self.card_size,
        })
        .insert_resource(Pointer::default())
        .insert_resource(LastAutoZ(self.initial_auto_z_value.or(Some(0.0)).unwrap()))
        .add_event::<DraggingStartedEvent>()
        .add_event::<DraggingStoppedEvent>()
        .add_systems(Update, (update_pointer, hoverable, draggable, dragging));
    }
}

impl BevyCardsPlugin {
    pub fn new(card_width: f32, card_height: f32) -> Self {
        Self {
            card_size: Vec2::new(card_width, card_height),
            initial_auto_z_value: Some(0.0),
        }
    }
}
