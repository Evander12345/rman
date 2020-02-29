//! Provides the `Host` struct as well as some functions to interact utilize them.
use crate::help;
extern crate serde_derive;
use config::Config;

use serde_derive::{Serialize, Deserialize};
use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

/// Struct to store host data in aggregate while in mem.
/// # Examples
/// let host = Host {
///     alias: String::from("localhost"),                    // alias field denotes a nickname for the remote machine.
///     ip: String::from("127.0.0.1"),                       // ip field denotes the remote machines IP.
///     ssh_user: String::from("root"),                      // ssh_user field denotes the ssh user.
///     pk_path: String::from("~/.ssh/localhost.pem"),       // pk_path denotes the path to the ssh private key.
///     description: String::from("An optional description") // description is an optional field to provide a short description of the remote machine.
/// }
#[derive(Debug, Default, Deserialize, Serialize, Clone)]
pub struct Host {
    pub alias: String,          // Host alias for reference purposes.
    pub ip: String,             // Remote machine's ip address.
    pub ssh_user: String,       // User to attempt to connect to on remote machine.
    pub pk_path: String,        // Path to the private key for the ssh connection.
    pub description: String,    // Brief optional description of remote machine.
}

/// Identical to `Host` except the fields should be given multiple `Host`'s concatenated together.
#[derive(Debug, Default, Deserialize, Serialize, Clone)]
pub struct Hosts {
    pub aliases: String,          // Host alias for reference purposes.
    pub ips: String,             // Remote machine's ip address.
    pub ssh_users: String,       // User to attempt to connect to on remote machine.
    pub pk_paths: String,        // Path to the private key for the ssh connection.
    pub descriptions: String,    // Brief optional description of remote machine.
}

/// This function handles all `$rman host` commands.
pub fn base(args: std::vec::Vec<String>) {
    if args.len() < 3 {
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
    for host in get_hosts() {
        dbg!(host);
    }
}

// Runs rm_host(<String>)
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

// Runs save_host(<Host>)
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

/// This function writes a `Host` into the config file
fn save_host(to_save: Host) {
    // Load config into mem in the form of Vec<Host>
    let mut configuration = match get_hosts() {
        Ok(Result) => Result,
        Err(E) => panic!("Error getting hosts.")
    };

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
        println!("Host already in configuration file!");
    }

    // Save the host configuration
    try_save(configuration);
}

fn try_save(configuration: std::vec::Vec<Host>) -> std::io::Result<()>{
    // Open the config file.
    let mut save_file = File::create(Path::new("/home/evan/.config/rman/rman.toml"))?;
    // Write bundled host values into the file...
    let hosts = bundle_hosts(configuration);
    save_file.write_all(format!("alias = '{}'\nip = '{}'\nssh_user = '{}'\npk_path = '{}'\ndescription = '{}'", hosts.aliases, hosts.ips, hosts.ssh_users, hosts.pk_paths, hosts.descriptions).into_bytes().as_ref())?;
    save_file.sync_data()?;
    Ok(())
}

// Bundles hosts together into one <Hosts> which is identical to a <Host>
fn bundle_hosts(to_hosts: std::vec::Vec<Host>) -> Hosts {
    let mut aliases = String::from("");
    let mut ips = String::from("");
    let mut ssh_users = String::from("");
    let mut pk_paths = String::from("");
    let mut descriptions = String::from("");

    // Bundle all host fields into one string using the pipe character as the delimiter.
    for host in to_hosts.iter() {
        aliases.push_str(format!("{}{}",host.clone().alias.as_str(), "|").as_str());
        ips.push_str(format!("{}{}",host.clone().ip.as_str(), "|").as_str());
        ssh_users.push_str(format!("{}{}",host.clone().ssh_user.as_str(), "|").as_str());
        pk_paths.push_str(format!("{}{}",host.clone().pk_path.as_str(), "|").as_str());
        descriptions.push_str(format!("{}{}",host.clone().description.as_str(), "|").as_str());
    }

    // Remove trailing pipe from fields
    aliases = aliases[0..aliases.len() - 1].parse().unwrap();
    ips = ips[0..ips.len() - 1].parse().unwrap();
    ssh_users = ssh_users[0..ssh_users.len() - 1].parse().unwrap();
    pk_paths = pk_paths[0..pk_paths.len() - 1].parse().unwrap();
    descriptions = descriptions[0..descriptions.len() - 1].parse().unwrap();

    Hosts{
        aliases,
        ips,
        ssh_users,
        pk_paths,
        descriptions
    }
}

/// Loads hosts from the config file and into a `Vec<Host>`
/// # Examples
/// let hosts_as_vec std::vec::Vec<Host> = get_hosts();
fn get_hosts() -> Result<std::vec::Vec<Host>, Box<Error>> {
    let mut settings = Config::new();
    settings.merge(config::File::with_name("/home/evan/.config/rman/rman.toml"));

    let aliases: std::vec::Vec<String> = to_string_vec(settings.get::<String>("alias")?.split("|").collect());
    let ips: std::vec::Vec<String> = to_string_vec(settings.get::<String>("ip")?.split("|").collect());
    let users: std::vec::Vec<String> = to_string_vec(settings.get::<String>("ssh_user")?.split("|").collect());
    let pkpaths: std::vec::Vec<String> = to_string_vec(settings.get::<String>("pk_path")?.split("|").collect());
    let descs: std::vec::Vec<String> = to_string_vec(settings.get::<String>("description")?.split("|").collect());

    let mut r_hosts: std::vec::Vec<Host> = vec!();
    for i in 0..aliases.len() {
        r_hosts.push(Host {
            alias: String::from(aliases[i].clone()),
            ip: String::from(ips[i].clone()),
            ssh_user: String::from(users[i].clone()),
            pk_path: String::from(pkpaths[i].clone()),
            description: String::from(descs[i].clone()),
        });
    }
    Ok(r_hosts)
}

// Converts a vec<str> to a vec<String>
fn to_string_vec(as_an_str: std::vec::Vec<&str>) -> std::vec::Vec<String>  {
    let mut r_vec = std::vec::Vec::new();
    for elem in as_an_str.iter() {
        r_vec.push(String::from(*elem))
    }
    r_vec
}