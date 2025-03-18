fn shift_string(s: &str, n: isize) -> String {
    let len = s.len();
    let shift = ((n % len as isize) + len as isize) % len as isize;  // Зберігає зсув в межах довжини рядка
    
    let (left, right) = s.split_at(s.len() - shift as usize);  // Розбиваємо рядок
    format!("{}{}", right, left)  // Повертаємо новий рядок з зсувом
}

fn main() {
    let test_cases = [
        ("abcdef", 2, "efabcd"), // Зсув праворуч
        ("abcdef", -2, "cdefab"), // Зсув ліворуч
        ("hello", 1, "ohell"),    // Зсув праворуч на 1 символ
    ];
    
    for (input, shift, expected) in test_cases.iter() {
        let result = shift_string(input, *shift);
        println!("Shifted '{}' by {}: '{}'", input, shift, result);
        assert_eq!(result, *expected);
    }
}
