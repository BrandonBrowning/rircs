
#![feature(io)]

use std::io::{Result, Read, BufRead, BufReader, Write, BufWriter, BufStream};
use std::net::{TcpStream};

fn io() -> Result<()> {
    let mut raw_stream = try!(TcpStream::connect("irc.freenode.net:6667"));
    let mut stream = BufStream::new(raw_stream);

    stream.write("NICK rircs\r\n".as_bytes());
    stream.write("USER rircs localhost localhost :tutorial bot\r\n".as_bytes());
    stream.write("JOIN #tutbot-testing\r\n".as_bytes());
    stream.flush();

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
