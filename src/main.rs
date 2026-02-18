use std::env;
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};

fn handle_client(mut stream: TcpStream) {
    loop {
        let mut buff = [0; 1024];
        if let Err(err) = stream.read(&mut buff) {
            eprintln!("Failed to read from the stream \n Error: {}", err);
            std::process::exit(1);
        }

        println!("New Reqwest: {}", String::from_utf8_lossy(&buff));

        if let Err(err) = stream.write(&buff) {
            eprintln!("Falied to write to the stream \n Error: {}", err);
            std::process::exit(1);
        };
    }
}

fn main() {

    let addr = env::args()
        .nth(1)
        .unwrap_or_else(|| "127.0.0.1:8080".to_owned());


    let stream = TcpListener::bind(&addr).expect("Failed to bind");
    println!("listeneing on {}",addr);

    for stream in stream.incoming() {
        match stream {
            Ok(s) => handle_client(s),
            Err(err) => {
                println!("Failed to recieve the stream \n Error: {}", err);
                std::process::exit(1);
            }
        }
    }
}
