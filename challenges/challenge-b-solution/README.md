# Challenge B — Bronze-level referentie-implementatie

Voorbeeld-oplossing voor de Inventory Tracker challenge op Bronze-niveau. Alle gevraagde tests slagen, geen `unsafe`, geen `unwrap` of `expect` in productie-paden, atomiciteit gegarandeerd.

> **Spoiler-waarschuwing**: dit is een referentie-uitwerking. Probeer eerst zelf de puzzle in [`../challenge-b/`](../challenge-b/) voordat je verder leest.

## Runnen

```bash
cd challenge-b-solution
cargo test
```

Alle 9 tests moeten slagen.

## De kern van de oplossing — atomiciteit door scheiding

Het belangrijkste ontwerp-principe in deze implementatie zit in `remove`:

```rust
pub fn remove(&mut self, name: &str, count: u32) -> Result<u32, InventoryError> {
    // Stap 1 — VALIDATIE (read-only)
    let available = self.items.get(name).copied()
        .ok_or_else(|| InventoryError::UnknownProduct(name.to_string()))?;
    if count > available {
        return Err(InventoryError::InsufficientStock { requested: count, available });
    }

    // Stap 2 — MUTATIE (we weten dat het veilig is)
    let new_value = available - count;
    if let Some(entry) = self.items.get_mut(name) {
        *entry = new_value;
    }
    Ok(new_value)
}
```

De truc: we doen **alle validatie eerst** zonder ook maar iets aan de toestand te veranderen. Pas als we zeker weten dat de operatie kan slagen, muteren we. Daarmee is atomiciteit by-construction gegarandeerd — een falende `remove` kan niet halverwege de inventaris hebben aangepast.

Dit patroon werkt in Rust verrassend natuurlijk omdat het type-systeem je dwingt om expliciet te zijn over wat je leest versus wat je schrijft (`&self` versus `&mut self`).

## Ontwerp-keuzes

Drie expliciete keuzes die ik zou willen benoemen in een code review:

### 1. `u32` voor de voorraad — het type-systeem doet het werk

Door `u32` te kiezen voor voorraad-counts is "negatieve voorraad" letterlijk onuitspreekbaar. De compiler weigert je code als je een negatieve waarde probeert toe te kennen. Dat is een Rust-idiomatische manier om ongeldige toestanden uit te sluiten — geen runtime check nodig, geen `assert`, geen defensieve code.

### 2. `saturating_add` in `add` — geen stille overflow

```rust
*entry = entry.saturating_add(count);
```

Een Bronze-oplossing met `*entry += count;` zou ook werken — totdat een geintje-test 4 miljard items toevoegt en je `u32` wrapt naar nul. `saturating_add` plakt op `u32::MAX` in plaats van te wrappen. Geen paniek, geen stille data-corruptie, gewoon expliciet veilig.

### 3. Geen `unwrap` in productie-pad

In `remove` weten we na de validatie dat het product bestaat — `get_mut(name)` zal nooit `None` retourneren. Toch gebruiken we `if let Some(entry) = ...` in plaats van `.unwrap()`. Reden: ik wil dat een code-grep op `unwrap` in dit bestand níet de `remove`-functie raakt. Hygiëne.

De compiler optimaliseert de "onmogelijke" None-branch weg, dus dit kost geen performance.

## Wat deze oplossing NIET doet

Dit is bewust een Bronze-implementatie. Voor hogere niveaus ontbreken:

- **Silver — `transfer`**: een atomair `transfer(from, to, count)` die in één keer items verplaatst zonder tussenstaat. Patroon: valideer beide kanten eerst, muteer pas als beide kunnen.
- **Gold — thread-safety**: wrap de hele `Inventory` in `Arc<Mutex<...>>`. Race conditions zijn niet automatisch uitgesloten door Rust's borrow checker zodra je meerdere threads erbij betrekt.
- **Platinum — type-veilige producten**: vervang `String` keys door een `enum Product { Coffee, Tea, ... }`. Compile-time onmogelijk om een onbekend product op te vragen. Trade-off: minder flexibel, veel veiliger.

Voor wie de Gold-uitdaging wil aangaan: de truc bij `Arc<Mutex>` is om de `Mutex` zo kort mogelijk vast te houden. Houd het lock niet vast tijdens lang-lopende operaties, en gebruik nooit twee mutexes tegelijk zonder ordening (deadlock-risico).

## Wat is er Rust-idiomatisch aan deze oplossing?

Vier patronen die dit een idiomatische Rust-implementatie maken:

1. **`Result<T, E>` voor falende operaties.** Geen exceptions, geen sentinel-waarden — een type dat de compiler dwingt om elke fout-pad te behandelen.
2. **`Option<T>` voor "misschien aanwezig".** `get` retourneert `Option<u32>`, niet `-1` of `0`. De caller moet expliciet kiezen wat te doen bij afwezigheid.
3. **Custom enum voor fouten.** `InventoryError` heeft specifieke variants met velden — niet een opaque string-message. De caller kan op variant-niveau matchen en gericht reageren.
4. **`Default`-implementatie naast `new()`.** Standaard Rust-idiom: `new()` voor expliciete constructie, `Default::default()` voor wanneer het van het type kan worden afgeleid.

## Volledige bestandsstructuur

```
challenge-b-solution/
├── Cargo.toml
├── README.md           ← dit bestand
└── src/
    └── lib.rs          ← implementatie + 9 tests
```
