//! Provides remote command functionality for `Host` structs using the crate `ssh`

use crate::host;
extern crate ssh;
use ssh::*;
use std::path::Path;
use std::io::Read;

pub fn execute_remote_command(host: &host::Host, remote_cmd: &String) -> String {
    // Connect to the remote machine
    let mut session=Session::new().unwrap();
    session.set_host(host.ip.as_str()).unwrap();
    session.set_username(host.ssh_user.as_str()).unwrap();
    session.set_identity(Path::new(host.pk_path.as_str())).unwrap();
    let mut connected = false;
    let mut count: u8 = 0;
    while !connected || count > 2 {
        match session.connect() {
            Ok(_) => connected = true,
            Err(_) => count += 1 // Increment connection timeout counter...
        }
    }
    if connected {
        // Check to make sure the key can be used...
        let mut key_open = false;
        let mut key_count = 0;
        while !key_open {
            if key_count == 3 {
                break
            }
            match session.userauth_publickey_auto(None) {
                Ok(_) => key_open = true,
                Err(_) => println!("Password incorrect. Remaining tries: {}", 2 - key_count)
            }
            key_count += 1;
        }
        if !key_open {
            return String::from("Failed to open key.");
        }
        // Execute command on the remote machine...
        let mut s=session.channel_new().unwrap();
        s.open_session().unwrap();
        s.request_exec(remote_cmd.as_str().as_ref()).unwrap();
        s.send_eof().unwrap();
        let mut buf=Vec::new();
        while buf.is_empty() {
            match s.stdout().read_to_end(&mut buf) {
                Ok(_) => break,
                Err(_) => () // Attempt to read to buffer again...
            }
        }
        String::from(std::str::from_utf8(&buf).unwrap())
    } else {
        String::from("Host cannot be reached.")
    }
}

pub fn check_host(host: &host::Host) -> bool {
    // Attempt to connect to the remote host...
    let mut session=Session::new().unwrap();
    session.set_host(host.ip.as_str()).unwrap();
    session.set_username(host.ssh_user.as_str()).unwrap();
    session.set_identity(Path::new(host.pk_path.as_str())).unwrap();
    let mut connected = false;
    let mut count: u8 = 0;
    while !connected || count > 2 {
        match session.connect() {
            Ok(_) => connected = true,
            Err(_) => count += 1 // Increment connection timeout counter...
        }
    }
    std::mem::drop(session);
    connected
}

/// Checks if the ssh user has sudo privileges
pub fn check_privs(host: &host::Host) -> bool {
    let groups = execute_remote_command(host, &String::from("groups"));
    groups.contains("sudo")
}
/// Reboot the target host
pub fn reboot(host: &host::Host) {
    if check_privs(host) {
        execute_remote_command(host, &String::from("shutdown -r"));
    } else {
        println!("User lacks privileges to execute this command.");
    }
}
/// Shutdown the target host
pub fn shutdown(host: &host::Host) {
    if check_privs(host) {
        execute_remote_command(host, &String::from("shutdown"));
    } else {
        println!("User lacks privileges to execute this command.")
    }
}