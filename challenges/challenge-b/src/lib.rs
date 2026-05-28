// Challenge B — Memory-Safe Inventory Tracker
//
// Implementeer de Inventory struct zodat alle tests slagen.
// Zie ../challenge-b-inventory.md voor de volledige opdracht.

use std::collections::HashMap;

pub struct Inventory {
    // jouw datastructuur
}

impl Inventory {
    pub fn new() -> Self {
        todo!()
    }

    pub fn add(&mut self, name: &str, count: u32) -> u32 {
        todo!()
    }

    pub fn remove(&mut self, name: &str, count: u32) -> Result<u32, InventoryError> {
        todo!()
    }

    pub fn get(&self, name: &str) -> Option<u32> {
        todo!()
    }

    pub fn total(&self) -> u32 {
        todo!()
    }
}

#[derive(Debug, PartialEq)]
pub enum InventoryError {
    InsufficientStock { requested: u32, available: u32 },
    UnknownProduct(String),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add_and_get() {
        let mut inv = Inventory::new();
        inv.add("coffee", 100);
        assert_eq!(inv.get("coffee"), Some(100));
    }

    #[test]
    fn remove_within_stock() {
        let mut inv = Inventory::new();
        inv.add("coffee", 100);
        assert_eq!(inv.remove("coffee", 30), Ok(70));
    }

    #[test]
    fn remove_more_than_stock_fails_atomically() {
        let mut inv = Inventory::new();
        inv.add("coffee", 10);
        let err = inv.remove("coffee", 50);
        assert_eq!(
            err,
            Err(InventoryError::InsufficientStock { requested: 50, available: 10 })
        );
        assert_eq!(inv.get("coffee"), Some(10));
    }

    #[test]
    fn remove_unknown_product() {
        let mut inv = Inventory::new();
        assert_eq!(
            inv.remove("unknown", 1),
            Err(InventoryError::UnknownProduct("unknown".to_string()))
        );
    }

    #[test]
    fn total_across_products() {
        let mut inv = Inventory::new();
        inv.add("coffee", 100);
        inv.add("tea", 50);
        assert_eq!(inv.total(), 150);
    }
}
