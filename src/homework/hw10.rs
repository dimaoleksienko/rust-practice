fn is_palindrome(n: u32) -> bool {
    let original = n;
    let mut reversed = 0;
    let mut num = n;

    // Перевертаємо число
    while num > 0 {
        let digit = num % 10;
        reversed = reversed * 10 + digit;
        num /= 10;
    }

    // Перевіряємо, чи є число паліндромом
    original == reversed
}

fn main() {
    let test_cases = [121, 123, 444, 12321, 10];

    for &num in test_cases.iter() {
        println!("Is {} a palindrome? {}", num, is_palindrome(num));
    }
}
