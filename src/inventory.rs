use std::io;
use std::collections::HashMap;
use Key;

/// A player inventory
pub struct Inventory(pub ItemMap<bool>);

impl Inventory {
    /// Take a set of changes from a transition and update the
    /// inventory.
    ///
    /// # Panics
    ///
    /// Panics if it finds a key that isn't present in the inventory.
    pub fn merge_changes(&mut self, changes: &ItemMap<bool>) {
        for (key, present) in changes.iter() {
            // Check the key is already there
            if let None = self.0.insert(*key, *present) {
                panic!("Inventory keys must be added during init");
            }
        }
    }

    /// Checks a subset of the inventory for agreement. Returns true
    /// if all items match, false otherwise.
    ///
    /// # Panics
    ///
    /// Panics if it finds a key that isn't present in the inventory.
    pub fn check(&self, subset: &ItemMap<bool>) -> bool {
        for (subset_key, subset_val) in subset.iter() {
            match self.0.get(subset_key) {
                Some(val) => if val != subset_val {
                    return false;
                },
                None => panic!("Inventory keys must be added during init")
            }
        }
        true
    }

    /// Write a description of the contents of your inventory to the
    /// supplied writer.
    pub fn describe(&self, item_map: (), w: impl io::Write) -> io::Result<()> {
        writeln!(w, "Your inventory contains: ")?;
        let mut keys_iter = self.0.iter()
            .filter_map(|(key, val)| if *val {
                Some(key)
            } else {
                None
            });
        if let Some(key) = keys_iter.next() {
            item_map.write_short(key, &w)?;
        }
        for key in keys_iter {
            write!(w, ", ")?;
            item_map.write_short(key, &w)?;
        }
        Ok(())
    }
}

impl From<ItemMap<bool>> for Inventory {
    fn from(map: ItemMap<bool>) -> Self {
        Inventory(map)
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct ItemKey(pub Key);

/// All the items and whether they are present in the inventory.
pub type ItemMap<T> = HashMap<ItemKey, T>;
