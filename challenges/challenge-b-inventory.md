# Challenge B — Memory-Safe Inventory Tracker

Schatting tijd: 30-60 minuten. Vereiste kennis: ownership, `Result`, `HashMap`. Voor de Gold-level: ook `Arc`/`Mutex`.

## Opdracht

Bouw een mini-applicatie die een productvoorraad bijhoudt. Het moet onmogelijk zijn om een ongeldige toestand te creëren — geen negatieve voorraad, geen "verdwenen" items tijdens transacties, geen race conditions als je multi-threaded gaat.

Dit is bewust een ontwerp-uitdaging, geen syntax-puzzel. Memory safety in Rust is meer dan "geen segfault" — het is een houding waarbij ongeldige toestanden compile-time of via het type-systeem worden uitgesloten.

## Required API

```rust
use std::collections::HashMap;

pub struct Inventory {
    // jouw datastructuur
}

impl Inventory {
    /// Maak een nieuwe lege inventaris.
    pub fn new() -> Self {
        todo!()
    }

    /// Voeg `count` items toe van product `name`.
    /// Retourneert de nieuwe voorraad voor dat product.
    pub fn add(&mut self, name: &str, count: u32) -> u32 {
        todo!()
    }

    /// Verwijder `count` items van product `name`.
    /// Retourneert `Err` als de voorraad onvoldoende is of het product niet bestaat.
    /// Mag NOOIT een negatieve waarde produceren — gebruik het type-systeem.
    pub fn remove(&mut self, name: &str, count: u32) -> Result<u32, InventoryError> {
        todo!()
    }

    /// Geef de huidige voorraad van een product, of None.
    pub fn get(&self, name: &str) -> Option<u32> {
        todo!()
    }

    /// Geef de totale voorraad over alle producten.
    pub fn total(&self) -> u32 {
        todo!()
    }
}

#[derive(Debug, PartialEq)]
pub enum InventoryError {
    InsufficientStock { requested: u32, available: u32 },
    UnknownProduct(String),
}
```

## Test-suite die moet slagen

```rust
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
        // Belangrijk: voorraad mag niet zijn verlaagd na de fout
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
```

## Niveaus

### Bronze
Alle bovenstaande tests slagen. Geen `unsafe`. Geen `unwrap()` of `expect()` in productie-paden (test-code mag).

### Silver
Voeg een `transfer(from: &str, to: &str, count: u32)` toe die atomair items verplaatst. Tussenstaten waarin items "verdwenen" zijn (al weggehaald bij `from`, nog niet toegevoegd bij `to`) zijn niet toegestaan, ook niet bij fouten.

### Gold
Maak de hele structuur thread-safe. Wrap de interne state in `Arc<Mutex<...>>` of `Arc<RwLock<...>>`. Voeg tests toe die `remove` en `add` parallel uitvoeren met `std::thread::spawn` of `tokio::spawn`. Bewijs dat de voorraad nooit onder nul kan, ongeacht het aantal threads.

### Platinum
Vervang `String` keys door een `enum Product { Coffee, Tea, ... }`. Maak het compile-time onmogelijk om `add`/`remove` aan te roepen met een onbekend product. Reflecteer: wat verlies je aan flexibiliteit, en wat win je aan veiligheid? Voor welke usecase is welke aanpak passend?

## Hoe in te leveren

Fork de repo, maak je oplossing in `challenges/challenge-b/`, en stuur een pull request met je naam en welk niveau je hebt bereikt. Wie de Gold-versie inlevert met een schone implementatie en passende tests krijgt een biertje na de volgende sessie.

## Hint

De atomiciteit in Silver bereik je niet door rollback, maar door **alle validatie vooraf** te doen en mutaties pas uit te voeren als zeker is dat ze allemaal kunnen slagen. Een patroon dat in Rust verrassend mooi werkt — denk aan het scheiden van "checken" en "doen".
