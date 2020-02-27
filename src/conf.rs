extern crate confy;
extern crate serde_derive;

use serde_derive::{Serialize, Deserialize};

// Struct to store host data in aggregate while in mem.
#[derive(Debug, Deserialize, Serialize)]
pub struct Host {
    pub alias: String,          // Host alias for reference purposes.
    pub ip: String,             // Remote machine's ip address.
    pub ssh_user: String,       // User to attempt to connect to on remote machine.
    pub pk_path: String,        // Path to the private key for the ssh connection.
    pub description: String,    // Brief optional description of remote machine.
}

pub fn get_hosts() -> std::vec::Vec<Host> {
    let cfg = confy::load("rman");
    match cfg {
        Ok(res) => return res,
        Err(e) => panic!(e)
    }
}