fn main() {
    let pnt = Point { a: 5, b: 5, c: "c" };
    let a = pnt.a();
    let b = pnt.b();
    let c = pnt.c();
    println!("{a}::{b}::{c}")
}

struct Point<T1, T2> {
    a: T1,
    b: T1,
    c: T2,
}

impl<T, U> Point<T, U> {
    fn a(&self) -> &T {
        &self.a
    }
    fn b(&self) -> &T {
        &self.b
    }
    fn c(&self) -> &U {
        &self.c
    }
}
