extern crate tokio;
extern crate tokio_codec;
extern crate bytes;
extern crate tokio_io;

use std::{io, str, env, thread};
use std::time::Duration;
use bytes::*;
use tokio_io::codec::*;
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
            // Frame it to receive/send strings.
            let framed = LinesCodec::new().framed(stream);

            // Split stream into (stream::SplitSink, stream::SplitStream)
            let (tx, rx) = framed.split();

            let client = rx
	        .map(move |line| {
                    println!("Received message: {}", line);
                    line
                })
                .forward(tx)
                .and_then(|data| {
                    println!("socket received FIN packet and closed connection");
                    Ok(())
                })
                .map_err(|err| println!("error sending message"));

            tokio::spawn(client)
        });

    tokio::run(server);
}
