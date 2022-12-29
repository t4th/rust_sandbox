// tcp server
use std::str;

use std::{net::{TcpListener, TcpStream}, io::Read};

fn handle_client(stream: &mut TcpStream)
{
    let mut buffer: [u8; 100] = [0; 100];
    
    // todo: add loop to read all bytes to a vector
    let result = stream.read(&mut buffer);
    
    match result
    {
        Ok(v) => println!("{v:?}"),
        Err(e) => println!("{e:?}")
    }

    // make string slice from u8 array
    let test = str::from_utf8(&buffer).unwrap();
    println!("{test}");
}

fn main() -> std::io::Result<()>
{
    let listener = TcpListener::bind("127.0.0.1:80")?;

    // accept connections and process them serially
    for stream in listener.incoming()
    {
        handle_client(&mut stream?);
    }
    Ok(())
}