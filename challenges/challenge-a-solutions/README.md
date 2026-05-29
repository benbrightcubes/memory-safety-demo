# Challenge A — De drie correcte oplossingen

De `first_line` puzzle heeft minstens drie correcte oplossingen, allemaal in één regel, allemaal memory-safe by construction. Hieronder elke variant met code, korte uitleg en de subtiele verschillen.

> **Spoiler-waarschuwing**: dit zijn de antwoorden. Probeer eerst zelf de puzzle in [`../challenge-a/`](../challenge-a/) voordat je verder leest.

---

## Oplossing 1 — `split` + `next`

```rust
fn first_line(s: &str) -> &str {
    s.split('\n').next().unwrap_or("")
}
```

**Wat het doet**: splits de string op `'\n'` en pakt het eerste deel. `split` retourneert een iterator van slices — geen allocations. `.next()` geeft `Option<&str>`. `unwrap_or("")` levert een lege string voor het edge-case van een lege input.

**Eigenschappen**:
- One-liner, idiomatisch
- Werkt voor alle inputs inclusief lege string en string die met `\n` begint
- Geen unsafe, geen clone

> **`.unwrap_or()` is geen `.unwrap()`.** De regel verbiedt `.unwrap()` en `.expect()` — de panic-varianten die je programma laten crashen bij `None`. `.unwrap_or("")` paniekt nooit; het geeft simpelweg de default terug. Het is een veilige `Option`-combinator en dus volledig toegestaan. Een `grep unwrap` matcht ze allebei, dus het lijkt verwarrend — maar het zijn fundamenteel verschillende methoden. Wie tijdens de talk muggenzift: wijs naar oplossing 2 hieronder, die nul unwrap-varianten gebruikt.

---

## Oplossing 2 — `find` + slice

```rust
fn first_line(s: &str) -> &str {
    match s.find('\n') {
        Some(i) => &s[..i],
        None => s,
    }
}
```

**Wat het doet**: zoekt de positie van de eerste `'\n'`. Als hij gevonden wordt, retourneer een slice van begin tot die positie. Anders: de hele string.

**Eigenschappen**:
- Expliciet en C-achtig in stijl — leesbaarder voor mensen die nog niet bekend zijn met Rust's iterator-idiomatiek
- `find` retourneert `Option<usize>` — typisch Rust patroon voor "maybe niet gevonden"
- Slice-indexing met `&s[..i]` is zero-cost, geen allocation
- Iets meer regels dan optie 1, maar didactisch helderder
- **Gebruikt nul unwrap-varianten** — de strikste interpretatie van de regel. Pak deze als iemand twijfelt of `.unwrap_or()` wel mag.

---

## Oplossing 3 — `lines` iterator

```rust
fn first_line(s: &str) -> &str {
    s.lines().next().unwrap_or("")
}
```

**Wat het doet**: gebruikt de standaard `lines()` iterator. Lijkt identiek aan oplossing 1, maar gedraagt zich subtiel anders.

**Eigenschappen**:
- One-liner, idiomatisch
- **Belangrijk verschil**: `.lines()` herkent zowel `\n` als `\r\n` als regeleinde, en strip de `\r` automatisch
- Voor `"hello\r\n"` retourneert deze oplossing `"hello"`, terwijl oplossing 1 en 2 retourneren `"hello\r"` (met de carriage return)
- Mooie discussie-haak: wat is "correct"? Voor cross-platform tekstverwerking is `.lines()` typisch wat je wilt. Voor exacte byte-positie binnen de input zijn opties 1 en 2 voorspelbaarder

---

## De Windows line-ending puzzel

Dit is het didactische gouden moment voor de talk. Drie correcte oplossingen die er identiek uitzien — maar gedragen zich verschillend op één concrete input.

```rust
let windows_line = "hello\r\nworld";

first_line_split(windows_line);  // → "hello\r"  (4 chars + \r)
first_line_find(windows_line);   // → "hello\r"  (4 chars + \r)
first_line_lines(windows_line);  // → "hello"    (4 chars)
```

Welke is "correct"? Het antwoord is: het hangt af van wat je probleem is.

- Heb je text die mogelijk uit een Windows-systeem komt en wil je de regel **inhoud**? Gebruik `.lines()`.
- Heb je de positie nodig binnen de originele string (bijvoorbeeld voor een parser of editor)? Gebruik `.find()` of `.split()`.
- Wil je gewoon een snelle implementatie zonder cross-platform zorgen? `.split('\n')` is meestal voldoende.

Dit illustreert een fundamenteel Rust-principe: **idiomatisch is niet altijd één antwoord**. De taal geeft je meerdere wegen, elk met eigen trade-offs.

---

## Wat deze challenge laat zien

1. **Slices zonder allocations zijn idiomatisch**. Geen `String`, geen `clone()`, geen heap-pressure. De Rust standaardbibliotheek is doordrenkt met iterators en slice-operaties die werken op de originele data.
2. **`Option`-combinators vervangen `unwrap()`**. `unwrap_or`, `map_or`, `and_then`, `filter` — een hele familie methoden die je `Option<T>` netjes laten afhandelen zonder paniek-risico.
3. **Lifetime elision werkt automatisch**. In alle drie oplossingen is de lifetime-relatie tussen input en output impliciet duidelijk voor de compiler. Geen `'a` annotaties nodig.
4. **Idioom kent meerdere correcte vormen**. Goede Rust-stijl is niet één antwoord — het is keuze tussen meerdere correcte antwoorden, gebaseerd op de specifieke usecase.

---

## Zelf uitvoeren

```bash
cd challenge-a-solutions
cargo run
```

De `main.rs` runt alle drie de implementaties tegen de standaard test-cases, plus de cross-platform `\r\n` edge-case waar de oplossingen uit elkaar lopen. De `tests/` directory bevat per oplossing een test-set die je apart kunt runnen met `cargo test`.
