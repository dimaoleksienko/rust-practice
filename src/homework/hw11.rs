use rand::Rng; // Для генерації випадкових чисел

// Генерація випадкового вектора
fn gen_random_vector(n: usize) -> Vec<i32> {
    let mut rng = rand::thread_rng();
    (0..n).map(|_| rng.gen_range(10..100)).collect()
}

// Функція для знаходження мінімальної пари сусідніх елементів
fn min_adjacent_sum(data: &[i32]) -> (i32, i32) {
    let mut min_sum = i32::MAX;
    let mut min_pair = (0, 0);

    for i in 0..data.len() - 1 {
        let sum = data[i] + data[i + 1];
        if sum < min_sum {
            min_sum = sum;
            min_pair = (data[i], data[i + 1]);
        }
    }
    
    min_pair
}

// Функція для виведення результату в консоль
fn print_results(data: &[i32], min_pair: (i32, i32)) {
    println!("Generated vector: {:?}", data);
    println!("Minimum adjacent pair: ({}, {})", min_pair.0, min_pair.1);
    println!("Sum of minimum pair: {}", min_pair.0 + min_pair.1);
}

fn main() {
    // Генеруємо вектор довжиною 20
    let vec = gen_random_vector(20);
    
    // Знаходимо мінімальну пару сусідніх елементів
    let min_pair = min_adjacent_sum(&vec);
    
    // Виводимо результат
    print_results(&vec, min_pair);
}
