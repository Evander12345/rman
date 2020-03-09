//! Provides the `Host` struct as well as some functions to interact utilize them.

use crate::help;
use crate::ssh_con;
extern crate serde_derive;
extern crate dirs;
use config::Config;
use serde_derive::{Serialize, Deserialize};
use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use crate::ssh_con::{execute_remote_command, shutdown, reboot};

/// Struct to store host data in aggregate while in mem.
/// # Examples
/// ```
/// let host = Host {
///     alias: String::from("localhost"),                    // alias field denotes a nickname for the remote machine.
///     ip: String::from("127.0.0.1"),                       // ip field denotes the remote machines IP.
///     ssh_user: String::from("root"),                      // ssh_user field denotes the ssh user.
///     pk_path: String::from("~/.ssh/localhost.pem"),       // pk_path denotes the path to the ssh private key.
///     description: String::from("An optional description") // description is an optional field to provide a short description of the remote machine.
/// }
/// ```
pub struct Host {
    pub alias: String,          // Host alias for reference purposes.
    pub ip: String,             // Remote machine's ip address.
    pub ssh_user: String,       // User to attempt to connect to on remote machine.
    pub pk_path: String,        // Path to the private key for the ssh connection.
    pub description: String,    // Brief optional description of remote machine.
}

/// Identical to `Host` except the fields should be given multiple `Host`'s concatenated together.
pub struct Hosts {
    pub aliases: String,          // Host alias for reference purposes.
    pub ips: String,             // Remote machine's ip address.
    pub ssh_users: String,       // User to attempt to connect to on remote machine.
    pub pk_paths: String,        // Path to the private key for the ssh connection.
    pub descriptions: String,    // Brief optional description of remote machine.
}

/// This function handles all `$rman host` cli commands.
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
            "status" => host_status(args),          // host "status"
            "add" => save_host_runner(args),    // host "add"
            "del" => rm_host_runner(args),      // host "del"
            "ls" => list_hosts(),               // host "ls"
            "exec" => run_host_cmd(args),       // host "exec"
            "reboot" => host_status(args),      // host "reboot"
            "shutdown" => shutdown_host(args),  // host "shutdown"
            _ => help::host()                   // If the user typed something wrong then display the host help message
        }
    }
}

/// Reboot the target host.
fn reboot_host(args: std::vec::Vec<String>) {
    if args.len() < 5 {
        let host = get_host_by_alias(args[3].to_string());
        reboot(&host);
    } else {
        help::host();
    }
}

/// Shutdown the target host.
fn shutdown_host(args: std::vec::Vec<String>) {
    if args.len() < 5 {
        let host = get_host_by_alias(args[3].to_string());
        shutdown(&host);
    } else {
        help::host();
    }
}

/// Prints host uptime, disk usage, and various io statistics.
fn host_status(args: std::vec::Vec<String>) {
    if args.len() < 5 {
        let host = get_host_by_alias(args[3].to_string());
        println!("{}", execute_remote_command(&host, &String::from("uptime")));
        println!("{}", execute_remote_command(&host, &String::from("df -h /")));
        println!("{}", execute_remote_command(&host, &String::from("iostat | head -n 4")));
    } else {
        help::host();
    }
}

/// Run a command on the target host.
fn run_host_cmd(args: std::vec::Vec<String>) {
    // Assemble command
    let mut cmd = String::new();
    if args.len() < 4 {
        help::host();
    } else {
        for i in 4..args.len() {
            cmd.push_str(format!("{} ", args[i]).as_str());
        }
        println!("{}", ssh_con::execute_remote_command(&get_host_by_alias((args[3]).parse().unwrap()), &cmd))
    }
}

/// Lists hosts
fn list_hosts() {
    for host in get_hosts().iter() {
        println!("Host Alias : {}\nHost IP : {}\nHost SSHUser : {}\nHost PK Path : {}\nHost Desc. : {}\n", host.alias, host.ip, host.ssh_user, host.pk_path, host.description);
    }
}

/// Parse args and pass to `host::rm_host()`
fn rm_host_runner(args: std::vec::Vec<String>) {
    // If the user has specified an alias to be removed then run the rm_host function with that alias. Else, display host help.
    match args.len() {
        4 => rm_host(String::from(&args[3])),
        _ => help::host(),
    }
}

/// Removes the target host from the configuration file.
fn rm_host(rm_alias: String) {
    let mut hosts = get_hosts();
    let mut rm_indice:i32 = -1;
    for i in 0..hosts.len() {
        if hosts[i].alias == rm_alias {
            rm_indice = i as i32;
        }
    }
    if rm_indice == -1 {
        println!("Host not found");
    } else {
        hosts.remove(rm_indice as usize);
        println!("{:?}", try_save(hosts));
    }
}

/// Parse args and pass to `host::save_host()`
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
    let mut configuration = get_hosts();

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

/// Attempt to save the configuration file.
fn try_save(configuration: std::vec::Vec<Host>) -> std::io::Result<()>{
    let mut path = match dirs::home_dir() {
        Some(buf) => buf,
        _ => panic!("Error getting home directory"),
    };
    path.push(Path::new(".config/rman/rman.toml"));
    // Open the config file.
    let mut save_file = File::create(path)?;
    // Write bundled host values into the file...
    let hosts = bundle_hosts(configuration);
    save_file.write_all(format!("alias = '{}'\nip = '{}'\nssh_user = '{}'\npk_path = '{}'\ndescription = '{}'", hosts.aliases, hosts.ips, hosts.ssh_users, hosts.pk_paths, hosts.descriptions).into_bytes().as_ref())?;
    save_file.sync_data()?;
    Ok(())
}

/// Bundles hosts together into one `Hosts`
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
pub fn get_hosts() -> std::vec::Vec<Host> {
    match try_get_hosts() {
        Ok(result) => result,
        Err(err) => create_cfg()
    }
}

/// Attempts to get hosts from the configuration file.
fn try_get_hosts() -> Result<std::vec::Vec<Host>, Box<dyn Error>> {
    let mut settings = Config::new();
    let mut path = match dirs::home_dir() {
        Some(buf) => buf,
        _ => panic!("Error"),
    };
    path.push(Path::new(".config/rman/rman.toml"));
    settings.merge(config::File::from(path));

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

/// Gets a singular host by alias
/// # Examples
/// let localhost: Host = get_host_by_alias(String::from("localhost"));
fn get_host_by_alias(alias: String) -> Host {
    let hosts = get_hosts();
    match hosts.into_iter().find(|host| host.alias == alias) {
        Some(host) => host,
        None => panic!("host not found.")
    }
}

/// Converts a vec<str> to a vec<String>
fn to_string_vec(as_an_str: std::vec::Vec<&str>) -> std::vec::Vec<String>  {
    as_an_str.into_iter().map(|elem| String::from(elem)).collect()
}

/// Creates a configuration file with filler host to prevent confy errors.
fn create_cfg() -> std::vec::Vec<Host> {
    match try_save(vec!(Host {
        alias: "filler".to_string(),
        ip: "filler".to_string(),
        ssh_user: "filler".to_string(),
        pk_path: "filler".to_string(),
        description: "filler".to_string()
    })) {
        Ok(_) => println!("New configuration file created."),
        Err(_) => panic!("Unable to create a configuration file.")
    }
    vec!(Host {
        alias: "filler".to_string(),
        ip: "filler".to_string(),
        ssh_user: "filler".to_string(),
        pk_path: "filler".to_string(),
        description: "filler".to_string()
    })
}