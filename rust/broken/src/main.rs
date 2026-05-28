// Rust-versie met dezelfde bug als demo.c — letterlijk dezelfde fout.
//
// Deze code compileert NIET. Dat is de hele demo.
//
// Probeer:
//   $ cargo build
//
// Verwachte output:
//   error[E0515]: cannot return reference to local variable `buffer`

fn get_username(user_id: u32) -> &str {
    let buffer = format!("user_{}", user_id);
    &buffer  // pointer naar lokaal geheugen — borrow checker stopt dit
}

fn main() {
    let name = get_username(42);
    println!("Naam: {}", name);
}
