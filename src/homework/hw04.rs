const HEIGHT: usize = 5; // Висота ромба 

fn main() {
    let mut diamond = String::new();
    let mid = HEIGHT / 2;

    for y in 0..HEIGHT {
        for x in 0..HEIGHT {
            if x + y == mid || y - x == mid || x - y == mid || x + y == mid * 3 {
                diamond.push('*');
            } else {
                diamond.push(' ');
            }
        }
        diamond.push('\n');
    }

    print!("{}", diamond);
}
