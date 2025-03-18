fn is_prime(n: u32) -> bool {
    if n <= 1 {
        return false; // Числа менші або рівні 1 не є простими
    }
    for i in 2..=(n as f64).sqrt() as u32 {
        if n % i == 0 {
            return false; // Якщо число ділиться на i, воно не просте
        }
    }
    true // Якщо не було знайдено дільника, то число просте
}

fn main() {
    let numbers = [1, 2, 3, 4, 5, 10, 13, 16, 17, 18];
    
    for &num in numbers.iter() {
        println!("Is {} prime? {}", num, is_prime(num));
    }
}
