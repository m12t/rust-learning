fn main() {
    let arr: [i32; 5] = [1, 2, 3, 4, 5];
    let arr2 = [42; 5];
    let tup: (u8, u8, i32) = (23, 27, -382);

    let y: i32 = if true { 88 } else { 44 };
    println!("if expressions return a value: {y}");

    println!("tup: {:?}", tup.0);
    println!("arr: {:?}", arr[0]);
    println!("arr2: {:?}", arr2[0]);
    let x = do_something(99);
    println!("do_something() returned {x}");
    heap_test();
}

fn do_something(x: i32) -> i32 {
    if x < 5 {
        println!("small num given");
    } else if x > 5 {
        println!("large num given");
    }
    println!("do_something() was called");
    return x;
}

fn heap_test() {
    let mut s = String::from("hello");
    {
        let r1 = &mut s;
    }
    let r2 = &mut s;

    println!("{}, {}", r1, r2);
}
