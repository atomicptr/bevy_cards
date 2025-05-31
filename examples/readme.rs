use bevy::prelude::*;
use bevy_cards::prelude::*;

const CARD_WIDTH: u32 = 100;
const CARD_HEIGHT: u32 = 144;

fn main() -> AppExit {
    App::new()
        .add_plugins(DefaultPlugins)
        // add the plugin with the default card size
        .add_plugins(BevyCardsPlugin::new(CARD_WIDTH as f32, CARD_HEIGHT as f32))
        .add_systems(Startup, setup)
        .run()
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2d);

    // spawn a green card
    commands.spawn((
        Card, // Card can be dragged and dropped
        Sprite {
            color: Color::srgb(0.0, 1.0, 0.0),
            custom_size: Some(Vec2::new(CARD_WIDTH as f32, CARD_HEIGHT as f32)),
            ..default()
        },
        Transform::from_xyz(-100.0, 0.0, 0.0),
    ));

    // spawn a blue card, thats somewhat smaller
    commands.spawn((
        Card,
        CardSize(CARD_WIDTH as f32 * 0.8, CARD_HEIGHT as f32 * 0.8),
        Sprite {
            color: Color::srgb(0.0, 0.0, 1.0),
            custom_size: Some(Vec2::new(CARD_WIDTH as f32 * 0.8, CARD_HEIGHT as f32 * 0.8)),
            ..default()
        },
        Transform::from_xyz(100.0, 0.0, 0.0),
    ));
}
