use std::net::{TcpListener, TcpStream};
use flume::{Sender, Receiver, unbounded};
use std::io::{Read, Write};

pub struct Server {
    channel: (Sender<i32>, Receiver<i32>),
}

impl Server {
    pub fn new() -> Self {
        Self {
            channel: unbounded(),
        }
    }

    pub fn start(&mut self, address: &str) {
        let receiver = self.channel.1.clone();
        let a = address.clone().to_string();

        std::thread::spawn(move || {
            let listener = TcpListener::bind(a).unwrap();
            listener.set_nonblocking(true).unwrap();

            loop {
                if receiver.try_recv().is_ok() {
                    break;
                }

                if let Ok((stream, _socket_address)) = listener.accept() {
                    std::thread::spawn(move || {
                        handle_client(stream);
                    });
                }

                std::thread::sleep(std::time::Duration::from_millis(20));
            }
        });
    }

    pub fn stop(&mut self) {
        self.channel.0.send(5).unwrap();
    }

}

fn handle_client(mut stream: TcpStream) {
    let mut buffer = [0; 1024];

    loop {
        // Read data from the client
        match stream.read(&mut buffer) {
            Ok(bytes_read) => {
                if bytes_read == 0 {
                    // Connection closed by the client
                    println!("Client disconnected");
                    break;
                }

                // Convert the received bytes to a string
                let client_data = String::from_utf8_lossy(&buffer[..bytes_read]);
                println!("Received data from client: {}", client_data);

                // Send a response back to the client
                let response = "Hello from the server!";
                if let Err(err) = stream.write_all(response.as_bytes()) {
                    println!("Error sending response: {}", err);
                    break;
                }
            }
            Err(err) => {
                println!("Error reading from client: {}", err);
                break;
            }
        }

        // Clear the buffer for the next read
        buffer = [0; 1024];
    }
}