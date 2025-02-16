pub mod client;
use std::env;
use std::net::{TcpListener, TcpStream};
use std::io::Read;
fn handle_client(mut stream: TcpStream) {
    let a = 3;
    println!("The Stream here is : {:?}", stream);
    let mut buffer = [0u8;1024];
    loop {
    let bytes_read = stream.read(&mut buffer).unwrap();
    println!("The information given to the current stream is {:?}", String::from_utf8_lossy(&buffer[..bytes_read]));
    println!("The global variable in this scope is a {:?}", a);
    }

}

fn main() {
    // need to properly address how I can take the ip_address and port as input
    let args: Vec<String> = env::args().collect();

    let query = &args[1]; // this needs to be passed after passing the command
    let file_path = &args[2]; // this needs to be passed after passing the command 
    // example : cargo script main.rs main file

    println!("Searching for {query}");
    println!("In file {file_path}");


    let listener = TcpListener::bind("127.0.0.1:8081").unwrap();

    
    for stream in listener.incoming(){
        let stream = stream.unwrap();
        handle_client(stream);
    }
}
