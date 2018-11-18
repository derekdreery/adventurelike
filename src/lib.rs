pub mod inventory;

use std::collections::HashMap;

use inventory::{Inventory, ItemMap};

pub type Key = u32;


/// A running game
pub struct Game {
    pub state: GameState,
    pub items: ItemMap<ItemData>,
    pub locations: Locations
}

pub struct Locations(HashMap<LocationKey, LocationData>);

pub struct LocationData {
    /// A list of descriptions that are searched until one matches
    /// the current state. The last of this list should be a catch-all.
    descriptions: Vec<Description>
}

/// A description of an item or a location
pub struct Description {
    /// This description is only valid if all these states match the
    /// global state.
    state: StateMap<bool>,
    /// The text of the description.
    short: String,
    /// A longer text of the description, when the item/location is
    /// studied in detail.
    long: String,
}

/// Static data on an item.
pub struct ItemData {
    /// The item's name for displaying in the inventory.
    pub name: String,
    /// A longer description for when you are looking at it
    /// specifically.
    pub description: String,
}

/// The game state
pub struct GameState {
    /// The current location as a key
    pub current_location: LocationKey,
    /// For each item, whether it is present or not.
    pub inventory: Inventory,
    /// All the possible states and whether they are active or not.
    pub world_state: StateMap<bool>,
}

/// A collection of states and whether they are active or not.
pub type StateMap<T> = HashMap<StateKey, T>;
/// Information required to test whether a transition is possible.
pub struct TransitionTest {
    /// All transitions are only possible at a single location
    pub location: LocationKey,
    /// The items that must be present in the inventory for the
    /// transition to be valid.
    pub required_items: ItemMap<bool>,
    /// All the states that must be active or not for this transition
    /// to be valid.
    pub world_state: StateMap<bool>,
}

/// A possible transition between states.
pub struct Transition {
    /// The command that corresponds to this transition
    pub cmd: String,
    /// When this transition is valid
    pub test: TransitionTest,
    /// All the item changes that happen during this transition.
    ///
    /// This will be merged with the inventory to update it.
    pub item_change: ItemMap<bool>,
    /// If the transition involves a location change, the new location.
    pub new_location: Option<LocationKey>,
    /// All the world state changes that happen during this transition.
    ///
    /// This will be merged with the world state to update it.
    pub state_change: StateMap<bool>,
}

// Newtype wrappers for keys.
#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct LocationKey(pub Key);
#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct StateKey(pub Key);
