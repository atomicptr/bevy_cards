use bevy::prelude::*;
use bevy_cards::prelude::BevyCardsPlugin;

pub const CARD_WIDTH: u32 = 100;
pub const CARD_HEIGHT: u32 = 144;
pub const ATLAS_SIZE: (u32, u32) = (15, 4);

#[derive(Resource)]
pub struct CardMaker(pub Handle<Image>, pub Handle<TextureAtlasLayout>);

impl CardMaker {
    pub fn make_card_sprite(&self, suit: CardSuit, rank: CardRank) -> Sprite {
        Sprite::from_atlas_image(
            self.0.clone(),
            TextureAtlas {
                layout: self.1.clone(),
                index: card_index(suit, rank),
            },
        )
    }

    pub fn make_slot(&self) -> Sprite {
        Sprite::from_atlas_image(
            self.0.clone(),
            TextureAtlas {
                layout: self.1.clone(),
                index: 14,
            },
        )
    }
}

pub struct CardMakerPlugin;

impl Plugin for CardMakerPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(BevyCardsPlugin::new(CARD_WIDTH as f32, CARD_HEIGHT as f32))
            .add_systems(PreStartup, setup_card_maker);
    }
}

fn setup_card_maker(
    mut commands: Commands,
    assets: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    let cards_handle = assets.load("CuteCards.png");
    let layout = TextureAtlasLayout::from_grid(
        UVec2::new(CARD_WIDTH, CARD_HEIGHT),
        ATLAS_SIZE.0,
        ATLAS_SIZE.1,
        None,
        None,
    );
    let layout_handle = texture_atlas_layouts.add(layout);
    commands.insert_resource(CardMaker(cards_handle.clone(), layout_handle.clone()));
}

#[derive(Clone)]
pub enum CardSuit {
    Spade,
    Diamond,
    Club,
    Heart,
}

#[derive(Clone)]
pub enum CardRank {
    Ace,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Joker,
    Queen,
    King,
}

pub fn card_index(suit: CardSuit, rank: CardRank) -> usize {
    let row = match suit {
        CardSuit::Spade => 0,
        CardSuit::Diamond => 1,
        CardSuit::Club => 2,
        CardSuit::Heart => 3,
    };

    let col = match rank {
        CardRank::Ace => 0,
        CardRank::Two => 1,
        CardRank::Three => 2,
        CardRank::Four => 3,
        CardRank::Five => 4,
        CardRank::Six => 5,
        CardRank::Seven => 6,
        CardRank::Eight => 7,
        CardRank::Nine => 8,
        CardRank::Ten => 9,
        CardRank::Joker => 10,
        CardRank::Queen => 11,
        CardRank::King => 12,
    };

    row * (ATLAS_SIZE.0 as usize) + col
}
