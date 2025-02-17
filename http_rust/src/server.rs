use std::io::Read;
use std::net::{TcpListener, TcpStream};
use std::{env, thread};
fn handle_client(mut stream: TcpStream) {
    let a = 3;
    println!("The Stream here is : {:?}", stream);
    let mut buffer = [0u8; 1024];
    loop {
        let bytes_read = stream.read(&mut buffer).unwrap();
        println!(
            "The information given to the current stream is {:?}",
            String::from_utf8_lossy(&buffer[..bytes_read])
        );
        println!("The global variable in this scope is a {:?}", a);
    }
}

fn main() {
    // need to properly address how I can take the ip_address and port as input
    let args: Vec<String> = env::args().collect();

    let port = &args[1]; // this needs to be passed after passing the command
                         // example : cargo script main.rs 8081

    println!("The port submitted is {port}");
    let ip = "127.0.0.1";
    let address = format!("{}{}{}", ip, ":", port); // alternatively can use
                                                    // ip.push_str(":{port}"); , this needs ip to be ip = String::from("127.0.0.1"), supposedly better to continue with heap allocation
    let listener = TcpListener::bind(address).unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();
        thread::spawn(|| {
            handle_client(stream);
            // Task 2 :
            // maybe use atomic references to keep track of threads,
            // understand how you can see how many threads are allocated,
            // how can they be gracefully shutdown or consumed
        });
    }
}
