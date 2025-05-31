pub mod behaviour;
pub mod components;
pub mod events;
pub mod plugin;
pub mod settings;

pub mod prelude {
    pub use crate::{components::*, events::*, plugin::BevyCardsPlugin};
}
