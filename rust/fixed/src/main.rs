// Rust-versie — correct.
//
// In plaats van een referentie naar lokaal geheugen retourneren we een
// owned String. Identieke functionaliteit, geen onveilige patronen,
// memory safety gegarandeerd door de compiler.
//
// Probeer:
//   $ cargo run
//
// Verwachte output:
//   Naam: user_42

fn get_username(user_id: u32) -> String {
    format!("user_{}", user_id)
}

fn main() {
    let name = get_username(42);
    println!("Naam: {}", name);
}
