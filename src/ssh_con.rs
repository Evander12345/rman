//! Provides remote command functionality for `Host` structs via SSH
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
    session.connect().unwrap();
    //println!("{:?}",session.is_server_known());
    session.userauth_publickey_auto(None).unwrap();
    // Execute command on the remote machine...
    let mut s=session.channel_new().unwrap();
    s.open_session().unwrap();
    s.request_exec(remote_cmd.as_str().as_ref()).unwrap();
    s.send_eof().unwrap();
    let mut buf=Vec::new();
    s.stdout().read_to_end(&mut buf).unwrap();
    String::from(std::str::from_utf8(&buf).unwrap())
}