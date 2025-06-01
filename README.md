# bevy_cards

A simple, no dependency bevy plugin for making card games

## Features

- Hover over, drag & drop cards
- Drop cards into slots

## How to use

```rs
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
        Card,
        Draggable, // to drag & drop the card, we need to add the "Draggable" component
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
        Draggable,
        CardSize(Vec2::new(CARD_WIDTH as f32 * 0.8, CARD_HEIGHT as f32 * 0.8)),
        Sprite {
            color: Color::srgb(0.0, 0.0, 1.0),
            custom_size: Some(Vec2::new(CARD_WIDTH as f32 * 0.8, CARD_HEIGHT as f32 * 0.8)),
            ..default()
        },
        Transform::from_xyz(100.0, 0.0, 0.0),
    ));
}
```

Run this example using ``cargo run --example readme``

![Readme example in action](./.github/example-readme.gif)

## Concepts

### Card

Card is a component enabling your entity to be hovered, dragged and dropped around

### Slot

Slots are places your card can be dropped into

## Bevy versions supported

`bevy_cards` is not using semver, every release uses the same major and minor versions as `bevy` while the patch part is reserved
for all kinds of changes, be it bug fixes or feature updates.

| bevy   | bevy_pixcam | branch |
|--------|-------------|--------|
| 0.16.x | 0.16.x      | master |

## License

MIT
