// The <help> module is used to display tips for using rman to stdout

pub fn base(args: Vec<String>) {
    if args.len() < 2 {
        // If the user does not specify a particular command to receive help with then print the general help message.
        println!("\nrman usage:\n{}\n{}\n{}\n{}",
                 "$rman status [args]\tdisplay some info, see\"$rman help status\" for more details...",
                 "$rman host [args] \tinteract with hosts, see \"$rman help host\" for more details...",
                 "$rman all [args] \tinteract with all hosts, see \"$rman help all\" for more details...",
                 "Tip: Most rman commands can be shortened\t$rman all [args] == $rman a [args]",
        );
    }
    else {
        // If the user specifies a command, display the help message for that command
        let cmd: &str = &args[2];
        match cmd {
            "status" => status(),   // Display status command help
            "host" => host(),       // Display host command help
            "all" => all(),         // Display all command help
            _ => base(args)         // Display general help message
        }
    }
}

// Status command help message
fn status() {
    println!("\nrman status usage:\n{}",
             "$rman status\tdisplays some general status information",
    );
}

// Host command help message
fn host() {
    println!("\nrman host usage:\n{}\n{}\n{}\n{}\n{}\n{}\n{}",
             "$rman host ls\tprints the host list and whether the host is up or not",
             "$rman host add [host-alias] [ssh-user] [ssh id file path] [ip]\tadds a host to the host list",
             "$rman host del [host-alias]\tremoves host from the host list",
             "$rman host status [host]\tdisplays a detailed status view of a host",
             "$rman host reboot [host]\treboots the host",
             "$rman host shutdown [host]\tshutdowns the host",
             "$rman host exec [host] [cmd]\t execute an arbitrary command on the host."
    );
}

// All command help message
fn all() {
    println!("\nrman all usage:\n{}",
             "$rman all status\tdisplays detailed status view of all hosts",
    );
}
