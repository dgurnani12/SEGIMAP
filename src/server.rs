use std::collections::HashMap;

use std::io::TcpListener;
use std::io::IoResult;

use config::Config;
use email::Email;
use user::User;

/// Holds configuration state and email->user map
pub struct Server {
    conf: Config,
    pub users: HashMap<Email, User>
}

impl Server {
    /// Create server to hold the Config and User HashMap
    pub fn new(conf: Config, users: HashMap<Email, User>) -> Server {
        Server {
            conf: conf,
            users: users
        }
    }

    /// Create a TCP listener on the server host and imap post
    pub fn imap_listener(&self) -> IoResult<TcpListener> {
        return TcpListener::bind(self.conf.host.as_slice(), self.conf.imap_port);
    }

    /// Create a TCP listener on the server host and lmtp port
    pub fn lmtp_listener<'r>(&self) -> IoResult<TcpListener> {
        return TcpListener::bind(self.conf.host.as_slice(), self.conf.lmtp_port);
    }

    /// Return the server's host
    pub fn host(&self) -> &String {
        &self.conf.host
    }
}
