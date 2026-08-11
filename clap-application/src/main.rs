mod args;

use args::Cli;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::new();
    cli.run()
}
