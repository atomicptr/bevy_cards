use bevy::{
    ecs::{
        component::Component,
        entity::Entity,
        event::{Event, EventWriter},
        query::{Has, With, Without},
        resource::Resource,
        system::{Commands, Query, Res, ResMut},
    },
    input::{ButtonInput, mouse::MouseButton},
    log::{debug, error},
    math::Vec2,
    render::camera::Camera,
    transform::components::{GlobalTransform, Transform},
    window::{PrimaryWindow, Window},
};

use crate::settings::BevyCardsSettings;

#[derive(Default, Debug, Resource)]
pub struct Pointer {
    pub x: f32,
    pub y: f32,
}

const AUTO_Z_DELTA: f32 = 0.00001;

#[derive(Default, Resource)]
pub struct LastAutoZ(pub f32);

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

#[derive(Event)]
pub struct DraggingStartedEvent(pub Entity, pub Vec2);

#[derive(Event)]
pub struct DraggingStoppedEvent(pub Entity, pub Vec2);

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
            return;
        } else {
            commands.entity(entity).remove::<CardHovering>();
        }
    }
}

pub fn draggable(
    mut commands: Commands,
    pointer: Res<Pointer>,
    mouse_input: Res<ButtonInput<MouseButton>>,
    mut auto_z: ResMut<LastAutoZ>,
    draggable: Query<Entity, (With<CardHovering>, With<CardDraggable>)>,
    dragging: Query<(Entity, Has<CardAutoZ>), With<CardDragging>>,
    mut ev_dragging_started: EventWriter<DraggingStartedEvent>,
    mut ev_dragging_stopped: EventWriter<DraggingStoppedEvent>,
) {
    if mouse_input.just_pressed(MouseButton::Left) {
        if let Some(entity) = draggable.iter().next() {
            debug!("Started dragging {:?} at {:?}", entity, pointer);
            commands.entity(entity).insert(CardDragging);

            ev_dragging_started.write(DraggingStartedEvent(
                entity,
                Vec2::new(pointer.x, pointer.y),
            ));
        }
    }

    if mouse_input.just_released(MouseButton::Left) {
        for (entity, has_autoz) in dragging.iter() {
            debug!("Stopped dragging {:?} at {:?}", entity, pointer);
            commands.entity(entity).remove::<CardDragging>();

            if has_autoz {
                auto_z.0 += AUTO_Z_DELTA;
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
