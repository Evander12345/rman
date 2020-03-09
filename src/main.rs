//! Provides CLI entry into rman.

mod help;
mod host;
mod all;
mod ssh_con;

// Imports
extern crate config;
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
            "status" | "s" => show_status(),    // Execute the status command.
            "host" | "h" => host::base(args),   // Execute a host command.
            "all" | "a" => all::base(args),     // Execute an all command. 
            "help" => help::base(args),         // Display rman commands.
            _ => help::base(vec!(String::new())),
        }
    }
    else {
        // Show base help message when supplied no args.
        help::base(args);
    }
}

fn show_status() {
    let hosts = host::get_hosts();
    println!("Number of registered hosts: {}", hosts.len());
    println!("Number of reachable hosts: {}", all::check_up_hosts());
}