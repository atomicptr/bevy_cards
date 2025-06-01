use bevy::{
    app::{App, Plugin, Update},
    math::Vec2,
};

use crate::{
    behaviour::{
        DragTargetPrevPosition, Pointer, clean_up_previous_slotted_state, draggable, dragging,
        hoverable, update_pointer,
    },
    events::{
        DragEndedEvent, DragStartedEvent, HoverEndedEvent, HoverStartedEvent, SlottedIntoEvent,
        UnknownSlotTargetEvent,
    },
    settings::BevyCardsSettings,
};

pub struct BevyCardsPlugin {
    pub card_size: Vec2,
}

impl Plugin for BevyCardsPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(BevyCardsSettings {
            card_size: self.card_size,
        })
        .insert_resource(Pointer::default())
        .insert_resource(DragTargetPrevPosition::default())
        .add_event::<DragStartedEvent>()
        .add_event::<DragEndedEvent>()
        .add_event::<HoverStartedEvent>()
        .add_event::<HoverEndedEvent>()
        .add_event::<SlottedIntoEvent>()
        .add_event::<UnknownSlotTargetEvent>()
        .add_systems(
            Update,
            (
                update_pointer,
                hoverable,
                draggable,
                dragging,
                clean_up_previous_slotted_state,
            ),
        );
    }
}

impl BevyCardsPlugin {
    pub fn new(card_width: f32, card_height: f32) -> Self {
        Self {
            card_size: Vec2::new(card_width, card_height),
        }
    }
}
