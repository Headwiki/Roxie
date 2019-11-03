#[macro_use]
extern crate clap;
use clap::App;

use std::net::{TcpListener, TcpStream, SocketAddr};
use std::thread;
use std::time::Duration;

fn main() {
    let yaml = load_yaml!("cli.yml");
    let _matches = App::from_yaml(yaml).get_matches();

    thread::spawn(move || {
        client_to_roxie("127.0.0.1:3003".parse().unwrap()).unwrap(); 
    });
    thread::spawn(move || {
        let _stream = TcpStream::connect("127.0.0.1:3003").unwrap();

        
    });
    loop{
        thread::sleep(Duration::from_secs(1));
    }
}

fn client_to_roxie(socket: SocketAddr) -> std::io::Result<()> {
    let listener = TcpListener::bind(socket)?;
    println!("Listening on {socket}", socket=socket);

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                thread::spawn(move || {
                    handle_client(stream);
                });
            }
            Err(_) => {
                println!("Error");
            }
        }
    }

    Ok(())
}

fn handle_client(_stream: TcpStream) {
    println!("Connection!");
}