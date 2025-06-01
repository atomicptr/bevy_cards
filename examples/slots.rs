use std::env;

use bevy::{prelude::*, window::WindowResolution};
use bevy_cards::prelude::*;

use common::card_maker::{CARD_HEIGHT, CARD_WIDTH, CardMaker, CardMakerPlugin, CardRank, CardSuit};

mod common;

const GROUP_PLAYER: u16 = 0;

fn main() -> AppExit {
    unsafe {
        env::set_var("RUST_LOG", "warn,bevy_cards=debug");
    }

    App::new()
        .add_plugins(
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "Bevy Card Effects".to_string(),
                        resolution: WindowResolution::new(1280.0, 800.0),
                        resizable: false,
                        ..default()
                    }),
                    ..default()
                })
                .set(ImagePlugin::default_nearest()),
        )
        .insert_resource(ClearColor(Color::srgb(1.0, 99.0 / 255.0, 149.0 / 255.0)))
        .add_plugins(CardMakerPlugin)
        .add_systems(Startup, setup)
        .add_systems(Update, card_hover)
        .run()
}

fn setup(mut commands: Commands, card_asset: Res<CardMaker>) {
    commands.spawn(Camera2d);

    let cards = vec![
        (CardSuit::Spade, CardRank::Ace),
        (CardSuit::Heart, CardRank::Queen),
        (CardSuit::Diamond, CardRank::Ten),
        (CardSuit::Club, CardRank::Joker),
    ];

    for (i, (suit, rank)) in cards.iter().enumerate() {
        commands.spawn((
            card_asset.make_card_sprite(suit.clone(), rank.clone()),
            Card,
            Draggable,
            SnapBack,
            SnapIntoSlot,
            Slottable(GROUP_PLAYER),
            Transform::from_xyz(-200.0 + (i * CARD_WIDTH as usize) as f32, -100.0, 0.0),
        ));
    }

    for i in 0..5 {
        commands.spawn((
            card_asset.make_slot(),
            Slot {
                size: Vec2::new(CARD_WIDTH as f32, CARD_HEIGHT as f32) * 0.25,
                slot_group: GROUP_PLAYER,
                ..default()
            },
            Transform::from_xyz(-200.0 + (i * CARD_WIDTH) as f32, 100.0, 0.0),
        ));
    }
}

fn card_hover(
    mut ev_hover_started: EventReader<HoverStartedEvent>,
    mut ev_hover_ended: EventReader<HoverEndedEvent>,
    mut ev_drag_started: EventReader<DragStartedEvent>,
    mut query: Query<&mut Transform, (With<Card>, With<Draggable>)>,
) {
    ev_hover_started.read().for_each(|ev| {
        if let Ok(mut transform) = query.get_mut(ev.0) {
            transform.scale.x = 1.1;
            transform.scale.y = 1.1;
        }
    });

    ev_hover_ended.read().for_each(|ev| {
        if let Ok(mut transform) = query.get_mut(ev.0) {
            transform.scale.x = 1.0;
            transform.scale.y = 1.0;
        }
    });

    ev_drag_started.read().for_each(|ev| {
        if let Ok(mut transform) = query.get_mut(ev.0) {
            transform.scale.x = 1.0;
            transform.scale.y = 1.0;
        }
    });
}
