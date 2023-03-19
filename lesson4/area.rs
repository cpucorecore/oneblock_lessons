use std::f32::consts::PI;

trait CanArea {
    fn area(self: &Self) -> f32;
}

fn area<T: CanArea>(shape: &T) -> f32 {
    shape.area()
}

struct Rec {
    l: f32,
    w: f32,
}

impl CanArea for Rec {
    fn area(self: &Self) -> f32 {
        self.l * self.w
    }
}

struct Circle(f32);

impl CanArea for Circle {
    fn area(self: &Self) -> f32 {
        0.5 * PI * self.0
    }
}

struct Tri {
    a: f32,
    h: f32,
}

impl CanArea for Tri {
    fn area(self: &Self) -> f32 {
        0.5 * self.a * self.h
    }
}

fn main() {
    let r = Rec { l: 11.1, w: 12.1 };
    let c = Circle(2.0);
    let t = Tri { a: 1.0, h: 2.0 };
    println!("{:?}", area(&r));
    println!("{:?}", area(&c));
    println!("{:?}", area(&t));
}
