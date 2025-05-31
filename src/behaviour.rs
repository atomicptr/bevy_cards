use bevy::{
    ecs::{
        entity::Entity,
        event::EventWriter,
        query::{Has, With, Without},
        resource::Resource,
        system::{Commands, Query, Res, ResMut},
    },
    input::{ButtonInput, mouse::MouseButton},
    log::{debug, error},
    math::{
        Rect, Vec2,
        bounding::{Aabb2d, BoundingVolume, IntersectsVolume},
    },
    render::camera::Camera,
    transform::components::{GlobalTransform, Transform},
    window::{PrimaryWindow, Window},
};

use crate::{
    events::{DraggingStartedEvent, DraggingStoppedEvent},
    prelude::{
        CardAutoZ, CardDraggable, CardDragging, CardHoverable, CardHovering, CardSize, CardSlot,
        CardSlottable,
    },
    settings::BevyCardsSettings,
};

#[derive(Default, Debug, Resource)]
pub struct Pointer {
    pub x: f32,
    pub y: f32,
}

const AUTO_Z_DELTA: f32 = 0.00001;

#[derive(Default, Resource)]
pub struct OriginalPosition(pub Vec2);

#[derive(Default, Resource)]
pub struct LastAutoZ(pub f32);

pub fn update_pointer(
    mut pointer: ResMut<Pointer>,
    window: Query<&Window, With<PrimaryWindow>>,
    camera: Query<(&Camera, &GlobalTransform)>,
) {
    let camera = camera.single();
    if camera.is_err() {
        error!("could not get camera: {:?}", camera.unwrap_err());
        return;
    }

    let (camera, camera_transform) = camera.unwrap();

    let window = window.single();
    if window.is_err() {
        error!("could not get window: {:?}", window.unwrap_err());
        return;
    }

    let window = window.unwrap();

    if let Some(pos) = window
        .cursor_position()
        .and_then(|cursor| camera.viewport_to_world(camera_transform, cursor).ok())
        .map(|ray| ray.origin.truncate())
    {
        pointer.x = pos.x;
        pointer.y = pos.y;
    }
}

pub fn hoverable(
    mut commands: Commands,
    pointer: Res<Pointer>,
    settings: Res<BevyCardsSettings>,
    hoverable: Query<
        (Entity, &Transform, Option<&CardSize>),
        (With<CardHoverable>, Without<CardDragging>),
    >,
) {
    for (entity, transform, card_size) in hoverable.iter() {
        let half_width = card_size.map(|c| c.0).unwrap_or(settings.card_size.x) * 0.5;
        let half_height = card_size.map(|c| c.1).unwrap_or(settings.card_size.y) * 0.5;

        let hovering = transform.translation.x - half_width < pointer.x
            && transform.translation.x + half_width > pointer.x
            && transform.translation.y - half_height < pointer.y
            && transform.translation.y + half_height > pointer.y;

        if hovering {
            commands.entity(entity).insert(CardHovering);
        } else {
            commands.entity(entity).remove::<CardHovering>();
        }
    }
}

pub fn draggable(
    mut commands: Commands,
    settings: Res<BevyCardsSettings>,
    pointer: Res<Pointer>,
    mouse_input: Res<ButtonInput<MouseButton>>,
    mut original_position: ResMut<OriginalPosition>,
    mut auto_z: ResMut<LastAutoZ>,
    query_draggable: Query<
        (Entity, &Transform),
        (
            With<CardHovering>,
            With<CardDraggable>,
            Without<CardDragging>,
        ),
    >,
    mut query_dragging: Query<
        (
            Entity,
            &mut Transform,
            Has<CardAutoZ>,
            Option<&CardSlottable>,
            Option<&CardSize>,
        ),
        (With<CardDraggable>, With<CardDragging>),
    >,
    query_card_slots: Query<
        (Entity, &Transform, &CardSlot),
        (
            With<CardSlot>,
            Without<CardHoverable>,
            Without<CardDraggable>,
        ),
    >,
    mut ev_dragging_started: EventWriter<DraggingStartedEvent>,
    mut ev_dragging_stopped: EventWriter<DraggingStoppedEvent>,
) {
    if mouse_input.just_pressed(MouseButton::Left) {
        if let Some((entity, transform)) = query_draggable.iter().next() {
            debug!("Started dragging {:?} at {:?}", entity, pointer);
            commands.entity(entity).insert(CardDragging);

            // remember original drag start position
            original_position.0.x = transform.translation.x;
            original_position.0.y = transform.translation.y;

            ev_dragging_started.write(DraggingStartedEvent(
                entity,
                Vec2::new(pointer.x, pointer.y),
            ));
        }
    }

    if mouse_input.just_released(MouseButton::Left) {
        for (entity, mut transform, has_autoz, slottable, card_size) in query_dragging.iter_mut() {
            debug!("Stopped dragging {:?} at {:?}", entity, pointer);
            commands.entity(entity).remove::<CardDragging>();

            if has_autoz {
                auto_z.0 += AUTO_Z_DELTA;
            }

            println!("slottable {:?}", slottable);

            // if is a slottable, return to original position if not slotted
            if slottable.is_some() {
                let slottable = slottable.unwrap();

                let half_width = card_size.map(|c| c.0).unwrap_or(settings.card_size.x) * 0.5;
                let half_height = card_size.map(|c| c.1).unwrap_or(settings.card_size.y) * 0.5;

                let mut can_place = false;

                for (slot_entity, slot_transform, slot) in query_card_slots.iter() {
                    // only allow slotting if they are of the same type
                    if slottable.0 != slot.1 {
                        continue;
                    }

                    let slot_aabb = Aabb2d::new(
                        Vec2::new(slot_transform.translation.x, slot_transform.translation.y),
                        slot.0 * 0.25,
                    );

                    let card_aabb = Aabb2d::new(
                        Vec2::new(transform.translation.x, transform.translation.y),
                        Vec2::new(half_width, half_height),
                    );

                    if !slot_aabb.intersects(&card_aabb) {
                        continue;
                    }

                    can_place = true;

                    // cuz the card is now slotted, move it onto the slot and remove draggable
                    transform.translation.x = slot_transform.translation.x;
                    transform.translation.y = slot_transform.translation.y;
                    transform.translation.z = slot_transform.translation.z + AUTO_Z_DELTA;
                    commands.entity(entity).remove::<CardDraggable>();
                }

                if !can_place {
                    transform.translation.x = original_position.0.x.clone();
                    transform.translation.y = original_position.0.y.clone();
                }
            }

            ev_dragging_stopped.write(DraggingStoppedEvent(
                entity,
                Vec2::new(pointer.x, pointer.y),
            ));
        }
    }
}

pub fn dragging(
    pointer: Res<Pointer>,
    auto_z: Res<LastAutoZ>,
    mut dragging: Query<(&mut Transform, Has<CardAutoZ>), With<CardDragging>>,
) {
    for (mut transform, has_autoz) in dragging.iter_mut() {
        // TODO: can we tween or modify this through some closure of sorts?
        transform.translation.x = pointer.x;
        transform.translation.y = pointer.y;

        if has_autoz {
            transform.translation.z = auto_z.0 + AUTO_Z_DELTA;
        }
    }
}
