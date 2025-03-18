// Структура для представлення прямокутника
struct Rectangle {
    width: f64,
    height: f64,
}

// Структура для представлення круга
struct Circle {
    radius: f64,
}

// Функція для обчислення площі прямокутника
fn calculate_rectangle_area(rect: &Rectangle) -> f64 {
    rect.width * rect.height
}

// Функція для обчислення площі кола
fn calculate_circle_area(circ: &Circle) -> f64 {
    std::f64::consts::PI * circ.radius * circ.radius
}

fn main() {
    // Створення прямокутника з шириною 5.0 і висотою 3.0
    let rect = Rectangle {
        width: 5.0,
        height: 3.0,
    };
    
    // Створення кола з радіусом 2.0
    let circ = Circle {
        radius: 2.0,
    };

    // Обчислення площ
    let rect_area = calculate_rectangle_area(&rect);
    let circ_area = calculate_circle_area(&circ);

    // Виведення результатів
    println!("Area of the rectangle: {}", rect_area);
    println!("Area of the circle: {}", circ_area);
}
