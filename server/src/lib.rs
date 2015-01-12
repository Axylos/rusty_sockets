#![crate_name = "server"]
extern crate getopts;
pub use self::user_socket_manager::Message;
pub use self::user_socket_manager::server::boot_server;
pub mod build_socket_addr;
pub mod user_socket_manager;
pub mod boot_server;
