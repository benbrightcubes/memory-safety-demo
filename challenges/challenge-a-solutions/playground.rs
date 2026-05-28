// Plak dit volledig in play.rust-lang.org en klik RUN.
// Alle drie de oplossingen krijgen een eigen naam zodat Rust ze accepteert
// (Rust staat geen function overloading toe — zelfde naam = compile error).

fn first_line_split(s: &str) -> &str {
    s.split('\n').next().unwrap_or("")
}

fn first_line_find(s: &str) -> &str {
    match s.find('\n') {
        Some(i) => &s[..i],
        None => s,
    }
}

fn first_line_lines(s: &str) -> &str {
    s.lines().next().unwrap_or("")
}

fn main() {
    let cases = [
        ("simple newline",       "hello\nworld"),
        ("no newline",           "no newline here"),
        ("empty string",         ""),
        ("starts with newline",  "\nstarts with newline"),
        ("Windows line ending",  "hello\r\nworld"),
    ];

    println!("{:<25} | {:<18} | {:<18} | {:<18}",
             "Input", "split", "find", "lines");
    println!("{}", "-".repeat(90));

    for (label, input) in cases.iter() {
        println!(
            "{:<25} | {:<18?} | {:<18?} | {:<18?}",
            label,
            first_line_split(input),
            first_line_find(input),
            first_line_lines(input),
        );
    }

    println!();
    println!("Let op de laatste regel — split en find behouden de \\r,");
    println!("lines strip hem. Beide gedragingen zijn correct, afhankelijk");
    println!("van wat je met de output gaat doen.");
}
