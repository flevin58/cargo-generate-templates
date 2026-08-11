// This module parses the command line arguments using clap

use clap::{Parser, Subcommand};

// The Commands enum holds all available commands to the application.
// You should add an entry for each command, having as parameter a structure
// that will hold all the flags and args for that command.
// In the example below there are two commands, 'one' and 'two'.
#[derive(Subcommand, Debug)]
enum Commands {
   One(OneArgs),
   Two(TwoArgs),
}

// Here below we define the available flags for the 'one' command.
#[derive(Parser, Debug)]
pub struct OneArgs {
    #[arg(required = false, long, short, default_value_t = false)]
    pub verbose: bool,
    #[arg(required = false, long, short, default_value_t = "John".to_string())]
    pub name: String,
    #[arg(required = false, long, default_value_t = 42)]
    pub age: i8,
}

#[derive(Parser, Debug)]
pub struct TwoArgs {
    #[arg(required = false, long, default_value_t = false)]
    pub verbose: bool,
    #[arg(required = false, long, default_value_t = "John".to_string())]
    pub name: String,
    #[arg(required = false, long, default_value_t = 42)]
    pub age: i8,
}

// The Cli structure will hold the command line args
// and will run the appropriate command 
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
   #[command(subcommand)] 
   command: Commands,
}   

impl Cli {
   pub fn new() -> Self {
       Cli::parse()
   }

   // Here is the actual program logic.
   // You can call here a function defined in another module, like so:
   // one::run(args), two::run(args)
   pub fn run(&self) -> Result<(), Box<dyn std::error::Error>> {
       match &self.command {
           Commands::One(args) => {
                println!("{:#?}", args);
            }
            Commands::Two(args) => {
               println!("{:#?}", args);
           }
       }
       Ok(())
   }
}
