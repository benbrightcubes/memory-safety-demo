# Challenge A — De Lifetime Puzzle

Schatting tijd: 5-10 minuten. Vereiste kennis: basis Rust syntax.

## Opdracht

Schrijf een functie `first_line` die de eerste regel uit een string retourneert (alles tot de eerste `\n`), of de hele string als er geen `\n` in zit. **Geen allocations** — return een slice.

```rust
fn first_line(s: &str) -> &str {
    // jouw implementatie
}

fn main() {
    assert_eq!(first_line("hello\nworld"), "hello");
    assert_eq!(first_line("no newline here"), "no newline here");
    assert_eq!(first_line(""), "");
    assert_eq!(first_line("\nstarts with newline"), "");
    println!("Alle tests slagen!");
}
```

## Regels

- Geen `String`, geen `to_string()`, geen `clone()`
- Geen externe crates — alleen `std`
- Geen `unwrap()` of `expect()`

## Bonus

Doe het in één regel.

## Test

```bash
cd challenges/challenge-a
cargo run
```

Wie ben je tegen het lijf gelopen aan een lifetime-fout? Aan het einde van de talk bespreken we de drie meest voorkomende oplossingen — er zijn meer manieren dan je denkt.
