#[macro_use]
extern crate clap;
use clap::App;

use std::io::{Read, Write};
use std::net::{SocketAddr, TcpListener, TcpStream};
use std::thread;
use std::time::Duration;

struct Connection {
    source: SocketAddr,
    destination: SocketAddr,
    //listener: Option<TcpListener>, // Source stream
    //stream: Option<TcpStream>,     // Destination stream
}

impl Connection {
    fn new(source: SocketAddr, destination: SocketAddr) -> Connection {
        Connection {
            source: source,
            destination: destination,
        }
    }

    fn run(&mut self) -> Result<(), std::io::Error> {
        let listener = TcpListener::bind(self.source)?;
        println!("Listening on {source}", source = self.source);

        for source_stream in listener.incoming() {
            match source_stream {
                Ok(source_stream) => {
                    self.forward_data(source_stream).unwrap();
                }
                Err(_) => {
                    println!("Error");
                }
            }
        }

        Ok(())
    }

    fn forward_data(&self, mut s_stream: TcpStream) -> std::io::Result<()> {
        let mut buffer = [0u8; 256];

        while match s_stream.read(&mut buffer) {
            Ok(size) => {
                if size != 0 {
                    println!("Received from source: {:?}", &buffer[0..size]);
                    let mut d_stream = TcpStream::connect(self.destination)?;
                    d_stream.write(&buffer[0..size]).unwrap();
                    println!("Sent buffer, awaiting reply...");

                    let mut buffer_repsonse = [0u8; 256];
                    match d_stream.read(&mut buffer_repsonse) {
                        Ok(_) => {
                            println!("Received from destination: {:?}", &buffer_repsonse[0..size]);
                            s_stream.write(&buffer_repsonse[0..size]).unwrap();
                        }
                        Err(e) => {
                            println!("Failed to receive data: {}", e);
                        }
                    }
                }
                true
            }
            Err(error) => panic!("{}", error),
        } {}

        Ok(())
    }

}

fn main() {
    let yaml = load_yaml!("cli.yml");
    let _matches = App::from_yaml(yaml).get_matches();

    let mut new_connection = Connection::new(
        "127.0.0.1:3003".parse().unwrap(), 
        "127.0.0.1:3003".parse().unwrap()
    );

    thread::spawn(move || {
        new_connection.run().unwrap();
    });

    thread::sleep(Duration::from_secs(1));
    client_test();

    loop {
        thread::sleep(Duration::from_secs(1));
    }
}


fn client_test() {
    match TcpStream::connect("127.0.0.1:3003") {
        Ok(mut stream) => {
            println!("Test: Successfully connected to server in port 3003");

            let msg = b"Hello!";

            stream.write(msg).unwrap();
            println!("Test: Sent Hello");

            let mut buffer = [0u8;256];

            match stream.read(&mut buffer) {
                Ok(size) => {
                    println!("Test: Got: {:?}", &buffer[0..size]);
                },
                Err(_) => {

                }
            }

        }
        Err(e) => {
            println!("Failed to connect: {}", e);
        }
    }
}
