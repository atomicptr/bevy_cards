use bevy::{
    ecs::{entity::Entity, message::Message},
    math::Vec2,
};

#[derive(Message)]
pub struct DragStartedMessage(pub Entity, pub Vec2);

#[derive(Message)]
pub struct DragEndedMessage(pub Entity, pub Vec2);

#[derive(Message)]
pub struct HoverStartedMessage(pub Entity);

#[derive(Message)]
pub struct HoverEndedMessage(pub Entity);

#[derive(Message)]
pub struct SlottedIntoMessage {
    pub card: Entity,
    pub slot: Entity,
}

#[derive(Message)]
pub struct UnknownSlotTargetMessage(pub Entity);

// deprecated type aliases

#[deprecated = "*Event got renamed to *Message due to naming change in Bevy 0.17"]
pub type DragStartedEvent = DragStartedMessage;

#[deprecated = "*Event got renamed to *Message due to naming change in Bevy 0.17"]
pub type DragEndedEvent = DragEndedMessage;

#[deprecated = "*Event got renamed to *Message due to naming change in Bevy 0.17"]
pub type HoverStartedEvent = HoverStartedMessage;

#[deprecated = "*Event got renamed to *Message due to naming change in Bevy 0.17"]
pub type HoverEndedEvent = HoverEndedMessage;

#[deprecated = "*Event got renamed to *Message due to naming change in Bevy 0.17"]
pub type SlottedIntoEvent = SlottedIntoMessage;

#[deprecated = "*Event got renamed to *Message due to naming change in Bevy 0.17"]
pub type UnknownSlotTargetEvent = UnknownSlotTargetMessage;
