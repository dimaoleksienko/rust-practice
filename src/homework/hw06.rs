const LEVELS: usize = 3; // Кількість трикутників у ялинці

fn main() {
    let mut tree = String::new();

    (1..=LEVELS).for_each(|level| {
        let height = level + 1;
        (0..height).for_each(|i| {
            let spaces = " ".repeat(LEVELS + 1 - i);
            let stars = "*".repeat(2 * i + 1);
            tree.push_str(&format!("{}{}\n", spaces, stars));
        });
    });

    // Малюємо стовбур
    let trunk_spaces = " ".repeat(LEVELS);
    tree.push_str(&format!("{}|\n", trunk_spaces));

    print!("{}", tree);
}
