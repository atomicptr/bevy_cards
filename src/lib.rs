mod behaviour;
mod components;
mod events;
mod plugin;
mod settings;

pub mod prelude {
    pub use crate::{components::*, events::*, plugin::BevyCardsPlugin};
}
