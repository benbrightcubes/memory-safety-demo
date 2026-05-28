// Demonstreert de drie oplossingen tegen dezelfde inputs.
// Loopt elke oplossing tegen de standaard test-cases plus de
// Windows line-ending edge case waar de oplossingen uit elkaar lopen.

use challenge_a_solutions::{first_line_find, first_line_lines, first_line_split};

fn main() {
    let cases = [
        ("hello\\nworld", "hello\nworld"),
        ("no newline here", "no newline here"),
        ("(empty string)", ""),
        ("\\nstarts with newline", "\nstarts with newline"),
        ("hello\\r\\nworld (Windows line ending)", "hello\r\nworld"),
    ];

    println!("{:<46} {:<20} {:<20} {:<20}", "Input", "split", "find", "lines");
    println!("{}", "-".repeat(106));

    for (label, input) in cases.iter() {
        let s = format!("{:?}", first_line_split(input));
        let f = format!("{:?}", first_line_find(input));
        let l = format!("{:?}", first_line_lines(input));
        println!("{:<46} {:<20} {:<20} {:<20}", label, s, f, l);
    }

    println!();
    println!("Let op de laatste regel — split en find behouden de \\r,");
    println!("lines strip hem. Beide gedragingen zijn correct, afhankelijk");
    println!("van wat je met de output gaat doen.");
}
