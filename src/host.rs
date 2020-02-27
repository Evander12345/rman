use crate::conf;
use crate::help;

use std::path::Path;
use conf::Host;



pub fn base(args: std::vec::Vec<String>) {
    if args.len() < 2 {
        // If the user does not specify a particular command to execute then print the host help message.
        help::host();
    }
    else {
        // If the user specifies a command, execute that command
        let cmd: &str = &args[2];
        //println!("{}", cmd);
        match cmd {                             // Run various commands based on user input...
            //"status" => host_status(),        // host "status"
            "add" => save_host_runner(args),    // host "add"
            "del" => rm_host_runner(args),      // host "del"
            "ls" => list_hosts(),               // host "ls"
            _ => help::host()                   // If the user typed something wrong then display the host help message
        }
    }
}

fn list_hosts() {
    println!("{:?}", conf::get_hosts())
}

fn rm_host_runner(args: std::vec::Vec<String>) {
    // If the user has specified an alias to be removed then run the rm_host function with that alias. Else, display host help.
    match args.len() {
        3 => rm_host(String::from(&args[2])),
        _ => help::host(),
    }
}

fn rm_host(rm_alias: String) {
    // TODO: Implement host erasing by alias...
}

fn save_host_runner(args: std::vec::Vec<String>) {
    // Check to make sure the user has specified all of the required fields.
    if args.len() < 6 {
        // Display the host help
        help::base(args);
    }
    else {
        // If optional description is specified...
        if args.len() > 7 {
            // Use a description.
            save_host(Host {
                alias: String::from(&args[3]),
                ip: String::from(&args[4]),
                ssh_user: String::from(&args[5]),
                pk_path: String::from(&args[6]),
                description: String::from(&args[7]),
            });
        } else {
            // Else use blank description.
            save_host(Host {
                alias: String::from(&args[3]),
                ip: String::from(&args[4]),
                ssh_user: String::from(&args[5]),
                pk_path: String::from(&args[6]),
                description: String::from(""),
            });
        }
    }
}

// Add a host to the config file...
fn save_host(to_save: Host) {
    // Load config into mem in the form of Vec<Host>
    let mut configuration = conf::get_hosts();

    // Check to see if the host already exists before saving it again.
    let mut exists: bool = false;
    for host in configuration.iter() {
        if to_save.alias == host.alias {
            exists = true;
        }
    }
    if !exists {
        configuration.push(to_save);
    } else {
        panic!("Host already in configuration file!");
    }

    // Save the host configuration
    match confy::store("rman", configuration) {
        Ok(_t) => (),
        Err(e) => panic!(e),
    }
}