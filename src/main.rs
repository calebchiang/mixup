struct Point<T, U> {
    x: T,
    y: U,
}

impl<T, U> Point<T, U> {
    fn mixup<V, W>(self, other: Point<V, W>) -> Point<T, W> {
        Point {
            x: self.x,
            y: other.y,
        }
    }
}

fn main() {
    let point1 = Point {
        x: 4.0,
        y: 5,
    };

    let point2 = Point {
        x: 3,
        y: 12.0,
    };

    let point3 = point1.mixup(point2);
    println!("x: {:.1}, y: {:.1}", point3.x, point3.y);
}
