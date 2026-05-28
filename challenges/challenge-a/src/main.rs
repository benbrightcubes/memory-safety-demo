// Challenge A — De Lifetime Puzzle
//
// Schrijf een functie die de eerste regel uit een string retourneert,
// of de hele string als er geen newline in zit. Geen allocations.
//
// Regels: geen String, geen to_string(), geen clone(), geen unwrap().
// Bonus: doe het in één regel.

fn first_line(s: &str) -> &str {
    // jouw implementatie hier
    todo!()
}

fn main() {
    assert_eq!(first_line("hello\nworld"), "hello");
    assert_eq!(first_line("no newline here"), "no newline here");
    assert_eq!(first_line(""), "");
    assert_eq!(first_line("\nstarts with newline"), "");
    println!("Alle tests slagen!");
}
