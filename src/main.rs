#[macro_use]
extern crate clap;
use clap::App;

use std::net::{TcpListener, TcpStream, SocketAddr};
use std::thread;
use std::time::Duration;
use std::io::{Read, Write};

struct connection_info {
    source: SocketAddr,
    destination: SocketAddr
}

fn main() {
    let yaml = load_yaml!("cli.yml");
    let _matches = App::from_yaml(yaml).get_matches();

    thread::spawn(move || {
        source_to_roxie("127.0.0.1:3003".parse().unwrap()).unwrap(); 
    });
    thread::spawn(move || {
        //let _stream = TcpStream::connect("127.0.0.1:3003").unwrap();
        client_test();

        
    });
    loop{
        thread::sleep(Duration::from_secs(1));
    }
}


fn connection (info: &connection_info) -> std::io::Result<()>{

    // Create stream to destination


    // Create listener for source

    Ok(())
}

fn source_to_roxie(socket: SocketAddr) -> std::io::Result<()> {
    let listener = TcpListener::bind(socket)?;
    println!("Listening on {socket}", socket=socket);

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                thread::spawn(move || {
                    roxie_to_destination(stream).unwrap();
                });
            }
            Err(_) => {
                println!("Error");
            }
        }
    }

    Ok(())
}

fn roxie_to_destination(mut stream: TcpStream) -> std::io::Result<()> {

    //let mut stream = TcpStream::connect(socket)?;
    let mut buffer = [0u8;64];

        while match stream.read(&mut buffer) {
            Ok(size) => { 
                if size != 0 {
                    println!("{:?}", &buffer[0..size]);
                    stream.write(&buffer[0..size]).unwrap();
                }
                true
             },
            Err(error) => {
                panic!("{}", error)
            }
        } {}

    Ok(())
}

fn client_test() {
    match TcpStream::connect("127.0.0.1:3003") {
        Ok(mut stream) => {
            println!("Successfully connected to server in port 3003");

            let msg = b"Hello!";

            stream.write(msg).unwrap();
            println!("Sent Hello, awaiting reply...");

            let mut data = [0 as u8; 6]; // using 6 byte buffer
            match stream.read_exact(&mut data) {
                Ok(_) => {
                    if &data == msg {
                        println!("Reply is ok!");
                    } else {
                        let text = std::str::from_utf8(&data).unwrap();
                        println!("Unexpected reply: {}", text);
                    }
                },
                Err(e) => {
                    println!("Failed to receive data: {}", e);
                }
            }
        },
        Err(e) => {
            println!("Failed to connect: {}", e);
        }
    }
}