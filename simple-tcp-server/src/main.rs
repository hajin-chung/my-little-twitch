use std::{
    net::{TcpListener, TcpStream},
    io::{Read, self, Write},
    str,
};

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        println!("Connection established!");
        handle_connection(stream);
    }
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 8192];
    let n = stream.read(&mut buffer[..]).unwrap();
    let str = str::from_utf8(&mut buffer).unwrap();

    println!("{n}: {str}");

    buffer = [0; 8192];
    io::stdin().read(&mut buffer);
    stream.write(&buffer);
}