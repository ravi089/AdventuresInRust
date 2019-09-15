extern crate tokio;
extern crate tokio_codec;
extern crate bytes;
extern crate tokio_io;

use std::{io, str, env, thread};
use std::time::Duration;
use bytes::*;
use tokio_io::codec::*;
use tokio::fs::File;
use tokio::codec::Decoder;
use tokio::net::TcpListener;
use tokio::prelude::*;
use tokio_codec::{Framed, LinesCodec, BytesCodec};
use std::net::SocketAddr;

fn main() {
    let addr = env::args().nth(1).unwrap_or("127.0.0.1:12345".to_string());
    let addr = addr.parse::<SocketAddr>().expect("Address parsing error");

    let listener = TcpListener::bind(&addr).expect("Connection error");
    println!("Listening on: {}", addr);

    let server = listener
        .incoming()
        .map_err(|err| println!("failed to accept socket; error= {:?}", err))
        .for_each(move |stream| {
            println!("Accepted connection from: {}", stream.peer_addr().unwrap());

            let task = tokio::fs::File::open("foo.txt")
                .map_err(|err| println!("error opening file"))
	        .and_then(|file| {
                    BytesCodec::new()
                        .framed(file)
                        .map(Into::into)
                        .forward(BytesCodec::new().framed(stream))
                        .map(|_| ())
                        .map_err(|err| println!("error reading file"))
                });

            tokio::spawn(task)
        });

    tokio::run(server);
}
