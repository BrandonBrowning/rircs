
extern crate bufstream;

use std::error::{Error};
use std::io;
use std::io::{BufRead, Write};
use std::net::{TcpStream};
use bufstream::BufStream;

fn io() -> io::Result<()> {
    let raw_stream = try!(TcpStream::connect("irc.freenode.net:6667"));
    let mut stream = BufStream::new(raw_stream);

    try!(stream.write("NICK rircs\r\n".as_bytes()));
    try!(stream.write("USER rircs localhost localhost :tutorial bot\r\n".as_bytes()));
    try!(stream.write("JOIN #tutbot-testing\r\n".as_bytes()));
    try!(stream.flush());

    let mut line = String::new();
    loop {
        let line_length = try!(stream.read_line(&mut line));

        if line_length <= 2 {
            break;
        }

        line.truncate(line_length - 2);

        println!("< {}", line);

        line.clear();
    }

    Ok(())
}

fn main() {
    match io() {
        Ok(()) => (),
        Err(e) => println!("Error: {}", e.description()),
    }
}
