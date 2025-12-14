use std::net::{TcpListener, TcpStream};
use std::io::{Read, Write};
use std::thread;
use std::sync::mpsc;
use std::sync::{Arc, Mutex};
use talking_rust::ServerMessage;

fn print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>());
}

fn handle_client(mut stream: TcpStream, id: usize, tx: mpsc::Sender<ServerMessage>) {
    let mut buffer = [0u8; 512];
    loop {
        let bytes = match stream.read(&mut buffer) {
            Ok(0) => break,
            Ok(n) => n,
            Err(_) => break,
        };

        let msg = String::from_utf8_lossy(&buffer[..bytes]).to_string();
        let _ = tx.send(ServerMessage::new(id, msg));
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Running the server...");

    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    let (tx, rx) = mpsc::channel::<ServerMessage>();
    let clients = Arc::new(Mutex::new(Vec::<(usize, TcpStream)>::new()));
    let clients_clone = Arc::clone(&clients);
    let mut next_id = 0;

    // Broadcast thread.
    thread::spawn(move || {
        while let Ok(msg) = rx.recv() {
            let mut clients = clients_clone.lock().unwrap();
            for (id, stream) in clients.iter_mut() {
                if *id != *msg.get_from() {
                    let _ = stream.write_all(msg.get_content().as_bytes());
                }
            }
        }
    });
    
    for stream in listener.incoming() {
        let stream = stream?;
        let client_id = next_id;
        next_id += 1;
        clients.lock().unwrap().push((client_id, stream.try_clone()?));
        let tx_clone = tx.clone();

        thread::spawn(move || {handle_client(stream, client_id, tx_clone)});
    }
    Ok(())
}
