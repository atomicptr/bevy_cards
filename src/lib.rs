pub mod behaviour;
pub mod plugin;
pub mod settings;

pub mod events {
    pub use crate::behaviour::DraggingStoppedEvent;
}

pub mod components {
    pub use crate::behaviour::{Card, CardAutoZ, CardDraggable, CardHoverable, CardSize};
}

pub mod prelude {
    pub use crate::{components::*, events::*, plugin::BevyCardsPlugin};
}
