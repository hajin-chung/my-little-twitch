use std::{
    io::Result,
    io::{prelude::*, self},
    net::TcpStream,
    str,
};

fn main() -> Result<()> {
    let mut stream = TcpStream::connect("127.0.0.1:7878").unwrap();

    let mut buffer: [u8; 8192] = [0; 8192];
    let n = io::stdin().read(&mut buffer).unwrap();
    println!("size: {n}");
    stream.write(&buffer)?;

    stream.read(&mut buffer);
    let str = str::from_utf8(&mut buffer).unwrap();
    println!("{str}");
    stream.shutdown(std::net::Shutdown::Both);

    Ok(())
}
