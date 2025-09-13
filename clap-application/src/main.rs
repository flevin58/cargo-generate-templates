use clap::{Parser, arg, command};

#[derive(Parser)]
#[command(about = "Test clap struct parser", long_about = None)]
struct Cli {
    #[arg(long, short, default_value = "two")]
    two: String,
    #[arg(long, short, default_value = "one")]
    one: String,
}

fn main() {
    let cli = Cli::parse();
    println!("two: {}", cli.two);
    println!("one: {}", cli.one);
}
