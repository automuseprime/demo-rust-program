use std::env;

/// A small demo: greet, count, and show ownership + error handling.
/// Run with: cargo run -- [name]
/// Example: cargo run -- Paul
fn main() {
    let name = env::args().nth(1).unwrap_or_else(|| "world".to_string());

    println!("Hello, {}! This is a demo Rust program.", name);
    println!();

    // Counting with a loop
    println!("Counting to 5:");
    for i in 1..=5 {
        println!("  {}", i);
    }
    println!();

    // Ownership: `greeting` is moved into `shout`, so it can't be used after.
    let greeting = format!("hello from {}", name);
    let shouted = shout(greeting);
    println!("{}", shouted);
    println!();

    // Error handling with Result, no panics for bad input.
    match parse_number("42") {
        Ok(n) => println!("Parsed 42 as: {} (doubled: {})", n, n * 2),
        Err(e) => println!("Could not parse: {}", e),
    }
    match parse_number("not-a-number") {
        Ok(n) => println!("Parsed: {}", n),
        Err(e) => println!("Could not parse 'not-a-number': {}", e),
    }
}

fn shout(text: String) -> String {
    text.to_uppercase() + "!"
}

fn parse_number(s: &str) -> Result<i32, String> {
    s.parse::<i32>().map_err(|e| e.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn shout_uppercases() {
        assert_eq!(shout("hello".to_string()), "HELLO!");
    }

    #[test]
    fn parse_number_ok() {
        assert_eq!(parse_number("42"), Ok(42));
    }

    #[test]
    fn parse_number_err() {
        assert!(parse_number("abc").is_err());
    }
}
