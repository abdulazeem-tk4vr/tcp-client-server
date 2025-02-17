use std::io::{self, Result, Write};
use std::net::TcpStream;

fn main() -> Result<()> {
    let mut stream =
        TcpStream::connect("127.0.0.1:8081").expect("Couldn't connect to the server...");

    println!("We have connected");

    loop {
        let mut input = String::new();
        let bytes_read = io::stdin().read_line(&mut input)?;
        println!("The size of the information given is {:?}", bytes_read);
        println!(
            "The information given to the current stream is {:?}",
            input.trim()
        );

        stream.write(input.as_bytes()).unwrap();
        /*
        Task 1 and 3 : 
        this script is just sending information to the port
        I need to create a script that can allow a port to listen for new information and then send information back to where it received it from
        maybe create a listener object that actively listens for new connections and then prints them to the console
        create 2 functions handled by 2 separate threads :
            - sender : actively listens in a loop for incoming commands from the user and then sends it to the relevant port/s
            - listener : actively listens for information from other ports and just silently executes or sends information back if needed
        */
    }
}

