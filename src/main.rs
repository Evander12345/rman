mod help;

// Imports
use std::env;

fn main() {
    // Collect arguments.
    let args: Vec<String> = env::args().collect();

    // Handle user interaction via CLI...
    if args.len() > 1 {
        let cmd: &str = &args[1];
        match cmd {
            //"status" | "s" => status::base(), // TODO:Execute status command.
            //"host" | "h" => host::host(args), // TODO:Execute base host command.
            //"all" | "a" => all::all(args),    // TODO:Execute base all command.
            "help" | _ => help::base(args),     // Display rman commands.
        }
    }
    else {
        // Show base help message when rman is run without args.
        help::base(args);
    }
}