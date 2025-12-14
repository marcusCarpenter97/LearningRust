use std::net::TcpStream;
use std::io;
use std::io::{Write, Read};
use std::thread;

fn read_from_server(mut stream: &TcpStream) {
    let mut buffer = [0; 1024];

    loop {
        match stream.read(&mut buffer) {
            Ok(0) => {
                println!("Server disconected.");
                break;
            }
            Ok(size) => {
                println!("Message received: {}", String::from_utf8_lossy(&buffer[..size]));
            }
            Err(e) => {
                eprintln!("Read error: {}", e);
                break;
            }
        }
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Running the client...");
    let mut input = String::new();

    //println!("Enter username: ");
    //io::stdin().read_line(&mut input);

    let mut stream = TcpStream::connect("127.0.0.1:7878")?;

    let mut read_stream = stream.try_clone()?;

    // Thread for printing responces from server.
    let handle = thread::spawn(move || read_from_server(&read_stream));

    // Loop for user to interact with server.
    loop {
        input.clear();
        io::stdin().read_line(&mut input);

        stream.write_all(input.as_bytes());
        
    }
    Ok(())
}
