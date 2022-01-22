fn main() {
    let x: i32 = std::env::args().nth(1).expect("no x given").parse().expect("x must be integer");
    let y: i32 = std::env::args().nth(2).expect("no y given").parse().expect("y must be integer");

    println!("sum: {}", x + y);
}
