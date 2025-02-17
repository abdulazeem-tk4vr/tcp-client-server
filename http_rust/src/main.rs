use std::io::Result;

pub mod client;
pub mod server;

fn main() -> Result<()> {
    println!("This is the start of the main file in the http directory");
    Ok(())
}

/*
To Learn :
    - read about different types of Strings
    - read about write vs write_all
    - read about matching values Option and Result
    - read about why the stream object needs to be mutable
    - ? vs unwrap
    - heap vs stack allocation for strings, vars and objects
*/

/*
To Do :
    - Task 1 : response handling : ping pong implementation
    - Task 2 : add multi threading for handling connection responses
    - Task 3 : test with multiple clients
    - Task 4 : Make it look
*/

/*
Need to fix this when a client connection collapses :
Think about server connection as well.
PS C:\Relevant\work\dev\creative\http-rust\http_rust\src> cargo script server.rs 8081
The port submitted is 8081
The Stream here is : TcpStream { addr: 127.0.0.1:8081, peer: 127.0.0.1:56398, socket: 260 }
The information given to the current stream is "hu\r\n"
The global variable in this scope is a 3
The information given to the current stream is "nana\r\n"
The global variable in this scope is a 3
The information given to the current stream is "baba\r\n"
The global variable in this scope is a 3
The information given to the current stream is "bye\r\n"
The global variable in this scope is a 3
thread '<unnamed>' panicked at 'called `Result::unwrap()` on an `Err` value: Os { code: 10054, kind: ConnectionReset, message: "An existing connection was forcibly closed by the remote host." }', server.rs:9:51
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace

*/
