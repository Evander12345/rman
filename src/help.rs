//! Provides CLI help pages for interacting with rman

/// This function handles all "$rman help" commands.
pub fn base(args: Vec<String>) {
    if args.len() < 2 {
        // If the user does not specify a particular command to receive help with then print the general help message.
        println!("rman usage:\n{}\n{}\n{}",
                 "$rman status [args]\tdisplay some info, see\"$rman help status\" for more details...",
                 "$rman host [args] \tinteract with hosts, see \"$rman help host\" for more details...",
                 "$rman all [args] \tinteract with all hosts, see \"$rman help all\" for more details...",
        );
    }
    else {
        // If the user specifies a command, display the help message for that command
        let cmd: &str = &args[1];
        match cmd {
            "status" => status(),   // Display status command help
            "host" => host(),       // Display host command help
            "all" => all(),         // Display all command help
            _ => base(args)         // Display general help message
        }
    }
}

/// Status command help message, displays "$rman status" help page to stdout.
pub fn status() {
    println!("rman status usage:\n{}",
             "$rman status\tdisplays some general status information",
    );
}

/// Host command help message, displays "$rman host" help page to stdout.
pub fn host() {
    println!("rman host usage:\n{}\n{}\n{}\n{}\n{}\n{}\n{}\n{}",
             "$rman host ls\tprints the host list and whether the host is up or not",
             "$rman host add [host-alias] [ssh-user] [ssh id file path] [ip] [*description]\tadds a host to the host list",
             "$rman host del [host-alias]\tremoves host from the host list",
             "$rman host status [host-alias]\tdisplays a detailed status view of a host",
             "$rman host reboot [host-alias]\treboots the host",
             "$rman host shutdown [host-alias]\tshutdowns the host",
             "$rman host exec [host-alias] [cmd]\texecute an arbitrary command on the host.",
             "* denotes an optional argument."
    );
}

/// Host command help message, displays "$rman all" help page to stdout.
pub fn all() {
    println!("rman all usage:\n{}\n{}",
             "$rman all status\tdisplays a status view of all hosts",
             "$rman all exec [cmd]\texecutes a command on all remote hosts",
    );
}
