use std::net::TcpStream;
use std::io::{self, Result, Write};

fn main() -> Result<()> {
    let mut stream = TcpStream::connect("127.0.0.1:8081")
        .expect("Couldn't connect to the server...");

    println!("We have connected");

    loop {
        let mut input = String::new();
        let bytes_read = io::stdin().read_line(&mut input)?;
        println!("The information given to the current stream is {:?}", input.trim());

        stream.write(input.as_bytes()).unwrap();
    }
}



// read about different types of Strings
// read about write vs write_all
// read about matching values Option and Result
// read about why the stream object needs to be mutable
// ? vs unwrap