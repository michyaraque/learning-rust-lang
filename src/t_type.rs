#[allow(unused_variables)]
fn main() {
    let point_a: Point<i32> = Point {
        x: 0,
        y: 12,
    };

    let point_b: Point<f32> = Point {
        x: 0.59,
        y: 10.99,
    };

}

struct Point<T> {
    x: T,
    y: T,
}
