use clap::Parser;

#[derive(Parser)]
struct Cli {
    x: i32,
    y: i32,
}

fn main() {
    let args = Cli::parse();
    println!("sum: {}", args.x + args.y);
}
