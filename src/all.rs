//! Provides functions to interact with all hosts at once.

use crate::help;
use crate::host;
use crate::ssh_con::execute_remote_command;

/// This function handles all `$rman host` commands.
pub fn base(args: std::vec::Vec<String>) {
    if args.len() < 3 {
        // If the user does not specify a particular command to execute then print the host help message.
        help::all();
    }
    else {
        // If the user specifies a command, execute that command
        let cmd: &str = &args[2];
        //println!("{}", cmd);
        match cmd {                     // Run various commands based on user input...
            "status" => run_host_cmd(String::from("uptime")),// all "status"
            "exec" => exec_cmd(args),   // all "exec"
            _ => help::all()            // If the user typed something wrong then display the host help message
        }
    }
}

fn exec_cmd(args: std::vec::Vec<String>) {
    // Assemble the command from args
    let mut cmd = String::new();
    if args.len() < 3 {
        help::all();
    } else {
        for i in 3..args.len() {
            cmd.push_str(format!("{} ", args[i]).as_str());
        }
        run_host_cmd(cmd);
    }
}

fn run_host_cmd(cmd: String) {
    // Assemble each host's response into one string to print
    let mut print_string = String::new();
    let hosts = host::get_hosts();
    for host in hosts {
        print_string.push_str(format!("{}:\n{}\n", &host.alias, execute_remote_command(&host, &cmd)).as_str());
    }
    print!("{}", print_string);
}