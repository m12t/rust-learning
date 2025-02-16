fn main() {
    println!("Hello, world!");
    let out = combine("a", "b");
    println!("{}", out)
}

fn combine<T>(a: T, b: T) -> T {
    a + b
}
