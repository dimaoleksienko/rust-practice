// Структури для геометричних фігур

// Прямокутник
struct Rectangle {
    width: f64,
    height: f64,
}

// Коло
struct Circle {
    radius: f64,
}

// Трикутник
struct Triangle {
    base: f64,
    height: f64,
}

// Обчислення площі прямокутника
fn calculate_rectangle_area(rect: &Rectangle) -> f64 {
    rect.width * rect.height
}

// Обчислення площі кола
fn calculate_circle_area(circ: &Circle) -> f64 {
    std::f64::consts::PI * circ.radius * circ.radius
}

// Обчислення площі трикутника
fn calculate_triangle_area(tri: &Triangle) -> f64 {
    0.5 * tri.base * tri.height
}

// Функція для обчислення загальної площі, зайнятої усіма фігурами
fn calculate_total_area() -> f64 {
    // Створення фігур
    let rect = Rectangle {
        width: 5.0,
        height: 3.0,
    };
    
    let circ = Circle {
        radius: 2.0,
    };
    
    let tri = Triangle {
        base: 4.0,
        height: 3.0,
    };

    // Обчислення площ
    let rect_area = calculate_rectangle_area(&rect);
    let circ_area = calculate_circle_area(&circ);
    let tri_area = calculate_triangle_area(&tri);

    // Виведення площ
    println!("Area of the rectangle: {}", rect_area);
    println!("Area of the circle: {}", circ_area);
    println!("Area of the triangle: {}", tri_area);

    // Повернення загальної площі
    rect_area + circ_area + tri_area
}

fn main() {
    // Обчислюємо загальну площу
    let total_area = calculate_total_area();
    
    // Виведення загальної площі
    println!("Total occupied area: {}", total_area);
}
