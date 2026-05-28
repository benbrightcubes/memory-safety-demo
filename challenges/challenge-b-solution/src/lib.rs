//! Challenge B — Bronze-level referentie-implementatie.
//!
//! Een memory-safe inventaris-tracker. De belangrijkste les van deze
//! challenge is het scheiden van validatie en mutatie — eerst checken
//! of een operatie kan slagen, dan pas de toestand veranderen.
//! Daarmee is atomiciteit by-construction gegarandeerd: een falende
//! `remove` laat de voorraad onveranderd.

use std::collections::HashMap;

/// Een inventaris van producten, geïdentificeerd op naam.
///
/// Negatieve voorraad is onmogelijk doordat we `u32` als type kiezen
/// — het type-systeem garandeert dat we niet onder nul kunnen komen.
pub struct Inventory {
    items: HashMap<String, u32>,
}

impl Inventory {
    /// Maak een nieuwe, lege inventaris.
    pub fn new() -> Self {
        Inventory {
            items: HashMap::new(),
        }
    }

    /// Voeg `count` items toe van product `name`. Maakt het product aan
    /// als hij nog niet bestaat. Retourneert de nieuwe voorraad.
    ///
    /// Gebruikt `saturating_add` voor het pathologische geval van overflow
    /// — beter dan een paniek of stille wrap.
    pub fn add(&mut self, name: &str, count: u32) -> u32 {
        let entry = self.items.entry(name.to_string()).or_insert(0);
        *entry = entry.saturating_add(count);
        *entry
    }

    /// Verwijder `count` items van product `name`. Retourneert de nieuwe
    /// voorraad bij succes, of een `InventoryError` bij falen.
    ///
    /// CRUCIAAL: deze methode is atomair. Als de operatie faalt — omdat
    /// het product niet bestaat of onvoldoende voorraad heeft — wordt
    /// de inventaris niet aangepast. Dit lossen we op door VALIDATIE en
    /// MUTATIE te scheiden in twee duidelijke stappen.
    pub fn remove(&mut self, name: &str, count: u32) -> Result<u32, InventoryError> {
        // Stap 1 — VALIDATIE (read-only).
        // We kijken naar de huidige toestand en bepalen of de operatie
        // kan slagen. Geen mutaties in deze stap.
        let available = self
            .items
            .get(name)
            .copied()
            .ok_or_else(|| InventoryError::UnknownProduct(name.to_string()))?;

        if count > available {
            return Err(InventoryError::InsufficientStock {
                requested: count,
                available,
            });
        }

        // Stap 2 — MUTATIE.
        // Op dit punt weten we dat de operatie veilig is. We mogen muteren.
        let new_value = available - count;
        if let Some(entry) = self.items.get_mut(name) {
            *entry = new_value;
        }
        // De `if let` is theoretisch onnodig (we hebben net gecontroleerd dat
        // `name` bestaat), maar voorkomt elke vorm van `unwrap()` in de
        // productie-pad. De compiler optimaliseert dit weg.

        Ok(new_value)
    }

    /// Geef de huidige voorraad van een product, of `None` als het niet bestaat.
    pub fn get(&self, name: &str) -> Option<u32> {
        self.items.get(name).copied()
    }

    /// Geef de totale voorraad over alle producten.
    pub fn total(&self) -> u32 {
        // `sum()` op een iterator van `u32` kan in theorie overflowen.
        // Voor een Bronze-implementatie accepteren we dat — Gold-level
        // zou hier `checked_sum` of `saturating_sum` willen.
        self.items.values().sum()
    }
}

impl Default for Inventory {
    fn default() -> Self {
        Self::new()
    }
}

/// Fouten die `remove` kan retourneren.
#[derive(Debug, PartialEq)]
pub enum InventoryError {
    /// Er was onvoldoende voorraad om de gevraagde hoeveelheid te verwijderen.
    InsufficientStock { requested: u32, available: u32 },
    /// Het opgegeven product bestaat niet in de inventaris.
    UnknownProduct(String),
}

// ---- Tests --------------------------------------------------------------

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
            Err(InventoryError::InsufficientStock {
                requested: 50,
                available: 10
            })
        );
        // CRUCIAAL: de voorraad mag NIET zijn verlaagd na de fout
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

    #[test]
    fn add_multiple_times_accumulates() {
        let mut inv = Inventory::new();
        inv.add("coffee", 30);
        inv.add("coffee", 70);
        assert_eq!(inv.get("coffee"), Some(100));
    }

    #[test]
    fn remove_to_zero_then_continue_works() {
        let mut inv = Inventory::new();
        inv.add("coffee", 50);
        assert_eq!(inv.remove("coffee", 50), Ok(0));
        // Product bestaat nog steeds, met voorraad 0
        assert_eq!(inv.get("coffee"), Some(0));
        // Verder verwijderen faalt netjes
        assert_eq!(
            inv.remove("coffee", 1),
            Err(InventoryError::InsufficientStock {
                requested: 1,
                available: 0
            })
        );
    }

    #[test]
    fn get_unknown_returns_none() {
        let inv = Inventory::new();
        assert_eq!(inv.get("ghost"), None);
    }

    #[test]
    fn total_on_empty_is_zero() {
        let inv = Inventory::new();
        assert_eq!(inv.total(), 0);
    }
}
