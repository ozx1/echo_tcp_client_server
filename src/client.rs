use std::env;
use std::io::{self, Read, Write};
use std::net::TcpStream;

fn main() {
    let addr = env::args()
        .nth(1)
        .unwrap_or_else(|| "127.0.0.1:8080".to_owned());

    match TcpStream::connect(&addr) {
        Ok(mut s) => {
            println!("Connected to the server");
            loop {
                let mut input = String::new();
                io::stdin()
                    .read_line(&mut input)
                    .expect("Failed to read the line");
                let input = input.as_bytes();
                if let Err(e) = s.write_all(&input) {
                    eprintln!("failed to send to the server \n Error: {}", e)
                };

                let mut output_buff = [0; 1024];
                s.read(&mut output_buff).expect("failed to read");
                println!("{}", String::from_utf8_lossy(&output_buff));
            }
        }
        Err(e) => eprintln!("Failed to connect to the server \n Error: {}", e),
    }
}
