use bevy::prelude::Component;

#[derive(Component, PartialEq, Copy, Clone, Debug)]
pub enum Faction {
    Player,
    Hostile,
}

impl Faction {
    pub fn is_hostile(self, other: Faction) -> bool {
        self != other
    }
}
