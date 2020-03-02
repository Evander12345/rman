# rman - A simple server management tool written in Rust.

## Installation
**R**ust Server **Man**ager can be installed by running the following commands.

`$git clone https://github.com/Evander12345/rman.git`

followed by
 
`$cargo install --path=rman`
 
## Usage
#### Adding a host into the configuration file...

`rman host add [alias] [ip/domain] [user] [pk-path] [optional-description]`

##### Example

`rman host add localhost 127.0.0.1 root /home/root/.ssh/key this-is-localhost`

#### Deleting a host from the configuration file

`rman host del [alias]`

##### Example

`rman host del localhost`

#### Running a command on a remote server

`rman host exec [alias] [cmd]`

##### Example

`rman host exec localhost uptime`

#### Running a command on all registered hosts

`rman all exec [cmd]`

##### Example

`rman all exec uptime`




