fn main() {
    let x = std::env::args().nth(1).expect("no name given");
    println!("Hello, {}!", x);
}
