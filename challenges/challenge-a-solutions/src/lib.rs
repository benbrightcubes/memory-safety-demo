//! Challenge A — drie correcte oplossingen voor `first_line`.
//!
//! Elke implementatie retourneert de eerste regel van een string,
//! of de hele string als er geen `\n` in zit. Geen allocations.

/// Oplossing 1 — split + next.
///
/// Splits de string op `'\n'`, pakt het eerste fragment. Werkt voor
/// alle inputs inclusief lege string. Eén regel, idiomatisch.
pub fn first_line_split(s: &str) -> &str {
    s.split('\n').next().unwrap_or("")
}

/// Oplossing 2 — find + slice.
///
/// Zoekt de positie van de eerste `'\n'` en sliced van daar.
/// Expliciet en C-achtig in stijl. Iets meer regels, didactisch helderder.
pub fn first_line_find(s: &str) -> &str {
    match s.find('\n') {
        Some(i) => &s[..i],
        None => s,
    }
}

/// Oplossing 3 — lines iterator.
///
/// Gebruikt `str::lines()`. SUBTIEL VERSCHIL: deze methode herkent
/// zowel `\n` als `\r\n` als regeleinde, en strip de `\r` automatisch.
/// Voor `"hello\r\nworld"` retourneert deze oplossing `"hello"`,
/// terwijl `split` en `find` retourneren `"hello\r"`.
pub fn first_line_lines(s: &str) -> &str {
    s.lines().next().unwrap_or("")
}

// ---- Tests ----------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    // De standaard test-cases die alle drie oplossingen moeten slagen.
    fn basic_cases<F: Fn(&str) -> &str>(first_line: F) {
        assert_eq!(first_line("hello\nworld"), "hello");
        assert_eq!(first_line("no newline here"), "no newline here");
        assert_eq!(first_line(""), "");
        assert_eq!(first_line("\nstarts with newline"), "");
    }

    #[test]
    fn split_passes_basic() {
        basic_cases(first_line_split);
    }

    #[test]
    fn find_passes_basic() {
        basic_cases(first_line_find);
    }

    #[test]
    fn lines_passes_basic() {
        basic_cases(first_line_lines);
    }

    // Cross-platform edge case — hier lopen de oplossingen uit elkaar.
    // Mooi didactisch moment voor de talk.

    #[test]
    fn split_keeps_carriage_return() {
        assert_eq!(first_line_split("hello\r\nworld"), "hello\r");
    }

    #[test]
    fn find_keeps_carriage_return() {
        assert_eq!(first_line_find("hello\r\nworld"), "hello\r");
    }

    #[test]
    fn lines_strips_carriage_return() {
        // Dit is het Windows line-ending verschil
        assert_eq!(first_line_lines("hello\r\nworld"), "hello");
    }
}
