use bevy::{
    ecs::{
        entity::Entity,
        event::{EventReader, EventWriter},
        query::{Has, With, Without},
        resource::Resource,
        system::{Commands, Query, Res, ResMut},
    },
    input::{ButtonInput, mouse::MouseButton},
    log::{debug, error},
    math::{
        Vec2,
        bounding::{Aabb2d, IntersectsVolume},
    },
    render::camera::Camera,
    transform::components::{GlobalTransform, Transform},
    window::{PrimaryWindow, Window},
};

use crate::{
    components::*,
    events::{
        DragEndedEvent, DragStartedEvent, HoverEndedEvent, HoverStartedEvent, SlottedIntoEvent,
        UnknownSlotTargetEvent,
    },
    settings::BevyCardsSettings,
};

#[derive(Default, Debug, Resource)]
pub struct Pointer {
    pub x: f32,
    pub y: f32,
}

const DRAG_Z_DELTA: f32 = 0.0001;

#[derive(Default, Debug, Resource)]
pub struct DragTargetPrevPosition {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

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
        (With<Card>, Without<Hovering>, Without<Dragging>),
    >,
    hovering: Query<
        (Entity, &Transform, Option<&CardSize>),
        (With<Card>, With<Hovering>, Without<Dragging>),
    >,
    mut ev_hover_started: EventWriter<HoverStartedEvent>,
    mut ev_hover_ended: EventWriter<HoverEndedEvent>,
) {
    for (entity, transform, card_size) in hoverable.iter() {
        let half_size =
            card_size.map_or(settings.card_size * 0.5, |card_size| card_size.half_size());

        let hovering = transform.translation.x - half_size.x < pointer.x
            && transform.translation.x + half_size.x > pointer.x
            && transform.translation.y - half_size.y < pointer.y
            && transform.translation.y + half_size.y > pointer.y;

        if hovering {
            debug!("Hover started for {:?}", entity);
            commands.entity(entity).insert(Hovering);
            ev_hover_started.write(HoverStartedEvent(entity));
        }
    }

    for (entity, transform, card_size) in hovering.iter() {
        let half_size =
            card_size.map_or(settings.card_size * 0.5, |card_size| card_size.half_size());

        let hovering = transform.translation.x - half_size.x < pointer.x
            && transform.translation.x + half_size.x > pointer.x
            && transform.translation.y - half_size.y < pointer.y
            && transform.translation.y + half_size.y > pointer.y;

        if !hovering {
            debug!("Hover ended for {:?}", entity);
            commands.entity(entity).remove::<Hovering>();
            ev_hover_ended.write(HoverEndedEvent(entity));
        }
    }
}

pub fn draggable(
    mut commands: Commands,
    settings: Res<BevyCardsSettings>,
    pointer: Res<Pointer>,
    mouse_input: Res<ButtonInput<MouseButton>>,
    mut previous_pos: ResMut<DragTargetPrevPosition>,
    query_draggable: Query<
        (Entity, &Transform),
        (
            With<Card>,
            With<Hovering>,
            With<Draggable>,
            Without<Dragging>,
        ),
    >,
    mut query_dragging: Query<
        (
            Entity,
            &mut Transform,
            Option<&CardSize>,
            Option<&Slottable>,
            Has<SnapBack>,
            Has<SnapIntoSlot>,
        ),
        (With<Card>, With<Dragging>),
    >,
    mut query_card_slots: Query<(Entity, &Transform, &mut Slot), (With<Slot>, Without<Card>)>,
    mut ev_dragging_started: EventWriter<DragStartedEvent>,
    mut ev_dragging_stopped: EventWriter<DragEndedEvent>,
    mut ev_slotted_into: EventWriter<SlottedIntoEvent>,
    mut ev_unknown_slot_target: EventWriter<UnknownSlotTargetEvent>,
) {
    if mouse_input.just_pressed(MouseButton::Left) {
        if let Some((entity, transform)) = query_draggable.iter().next() {
            debug!("Dragging started for {:?} at {:?}", entity, pointer);
            commands.entity(entity).insert(Dragging);

            // remember original drag start position
            previous_pos.x = transform.translation.x;
            previous_pos.y = transform.translation.y;
            previous_pos.z = transform.translation.z;

            ev_dragging_started.write(DragStartedEvent(entity, Vec2::new(pointer.x, pointer.y)));
        }
    }

    if mouse_input.just_released(MouseButton::Left) {
        for (
            entity,
            mut transform,
            card_size,
            slottable,
            should_snap_back,
            should_snap_into_slot,
        ) in query_dragging.iter_mut()
        {
            debug!("Dragging ended for {:?} at {:?}", entity, pointer);
            commands.entity(entity).remove::<Dragging>();

            // if is a slottable, return to original position if not slotted
            if slottable.is_some() {
                let slottable = slottable.unwrap();

                let half_size =
                    card_size.map_or(settings.card_size * 0.5, |card_size| card_size.half_size());

                let mut was_able_to_place = false;

                for (slot_entity, slot_transform, mut slot) in query_card_slots.iter_mut() {
                    // only allow slotting if they are of the same type
                    if slottable.0 != slot.slot_group {
                        continue;
                    }

                    // already has something in it
                    if slot.card.is_some() {
                        continue;
                    }

                    let slot_aabb = Aabb2d::new(
                        Vec2::new(slot_transform.translation.x, slot_transform.translation.y),
                        slot.size * 0.5,
                    );

                    let card_aabb = Aabb2d::new(
                        Vec2::new(transform.translation.x, transform.translation.y),
                        half_size,
                    );

                    if !slot_aabb.intersects(&card_aabb) {
                        continue;
                    }

                    was_able_to_place = true;

                    debug!("Slotted {:?} into {:?}", entity, slot_entity);

                    slot.card = Some(entity);

                    if should_snap_into_slot {
                        transform.translation.x = slot_transform.translation.x;
                        transform.translation.y = slot_transform.translation.y;
                        transform.translation.z = slot_transform.translation.z + DRAG_Z_DELTA;
                    }

                    ev_slotted_into.write(SlottedIntoEvent {
                        card: entity,
                        slot: slot_entity,
                    });

                    break;
                }

                if !was_able_to_place {
                    debug!("Failed to slot {:?}", entity);
                    ev_unknown_slot_target.write(UnknownSlotTargetEvent(entity));

                    if should_snap_back {
                        transform.translation.x = previous_pos.x;
                        transform.translation.y = previous_pos.y;
                        transform.translation.z = previous_pos.z;
                    }
                }
            }

            ev_dragging_stopped.write(DragEndedEvent(entity, Vec2::new(pointer.x, pointer.y)));
        }
    }
}

pub fn clean_up_previous_slotted_state(
    mut event_slotted_into: EventReader<SlottedIntoEvent>,
    mut query_slots: Query<(Entity, &mut Slot)>,
) {
    for ev in event_slotted_into.read() {
        // look for a slot where the entity ID is different but the current slotted item is the
        // same. This will only happen with previously occupied slots
        let slot = query_slots
            .iter_mut()
            .find(|(entity, slot)| *entity != ev.slot && slot.card == Some(ev.card));

        if slot.is_none() {
            continue;
        }

        let (_, mut slot) = slot.unwrap();

        slot.card = None;
    }
}

pub fn dragging(
    pointer: Res<Pointer>,
    original_position: Res<DragTargetPrevPosition>,
    mut dragging: Query<&mut Transform, With<Dragging>>,
) {
    for mut transform in dragging.iter_mut() {
        // TODO: can we tween or modify this through some closure of sorts?
        transform.translation.x = pointer.x;
        transform.translation.y = pointer.y;
        transform.translation.z = original_position.z + DRAG_Z_DELTA;
    }
}
