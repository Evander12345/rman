//! Provides CLI entry into rman.
mod help;
mod host;
mod ssh_con;
extern crate config;

// Imports
use std::env;

/// Main method handles arguments supplied via CLI
fn main() {
    // Collect arguments.
    let args: Vec<String> = env::args().collect();
    //println!("{:?}", args);
    // Handle user interaction via CLI...
    if args.len() > 1 {
        let cmd: &str = &args[1];
        match cmd {
            //"status" | "s" => status::base(), // Execute the status command TODO:Execute status command.
            "host" | "h" => host::base(args),   // Execute a host command. TODO:Finish all host commands.
            //"all" | "a" => all::all(args),    // Execute an all command. TODO:Execute base all command.
            "help" => help::base(args),         // Display rman commands.
            _ => help::base(vec!(String::from(""))),
        }
    }
    else {
        // Show base help message when supplied no args.
        help::base(args);
    }
}