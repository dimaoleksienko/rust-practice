use rand::Rng;

// Генерація випадкових вантажів для кораблів
fn gen_shipments(n: usize) -> Vec<u32> {
    let mut rng = rand::thread_rng();
    let mut shipments = vec![0; n];
    let total: u32 = (1..=n as u32).sum(); // Сума всіх елементів повинна бути кратною n

    // Генерація вантажів
    for i in 0..n {
        shipments[i] = rng.gen_range(1..=10); // Генерація випадкових вантажів
    }

    // Розподіл залишку
    let sum: u32 = shipments.iter().sum();
    let diff = total - sum;
    shipments[0] += diff; // Додаємо залишок до першого елемента, щоб забезпечити рівний розподіл

    shipments
}

// Функція для перевірки, чи можливо рівномірно розподілити вантаж
fn count_permutation(shipments: &Vec<u32>) -> usize {
    let total: u32 = shipments.iter().sum();
    let n = shipments.len();
    
    // Якщо сума вантажів не кратна кількості кораблів, рівномірний розподіл неможливий
    if total % n as u32 != 0 {
        return 0; // Неможливо рівномірно розподілити
    }

    let target = total / n as u32; // Цільова кількість вантажу на кожному кораблі
    let mut moves = 0;

    let mut shipments = shipments.clone();
    // Рахуємо кількість необхідних перенесень
    for i in 0..n {
        let diff = shipments[i] as i32 - target as i32;
        if diff > 0 {
            moves += diff.abs() as usize;
            shipments[i] -= diff.abs() as u32;
        }
    }

    moves
}

// Основна функція для тестування
fn main() {
    let n = 5; // Наприклад, 5 кораблів
    let shipments = gen_shipments(n);
    println!("Generated shipments: {:?}", shipments);

    let moves = count_permutation(&shipments);
    if moves == 0 {
        println!("It is not possible to distribute the shipments equally.");
    } else {
        println!("Minimum number of moves to distribute shipments equally: {}", moves);
    }
}
