fn swap_case(input: &str) -> String {
    input.chars()
        .map(|c| if c.is_lowercase() { c.to_ascii_uppercase() } else { c.to_ascii_lowercase() })
        .collect()
}

fn main() {
    let text = "Hello, Rust!";
    let swapped = swap_case(text);
    
    println!("Original: {}", text);
    println!("Swapped: {}", swapped);
}
