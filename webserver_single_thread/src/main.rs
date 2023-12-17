use std::{
    io::{BufRead, BufReader, Write},
    net::{TcpListener, TcpStream},
};

fn handle_connection(mut stream: TcpStream) -> std::io::Result<()> {
    let buf_reader = BufReader::new(&mut stream);
    let http_request = buf_reader
        .lines()
        .map(|result| result.unwrap())
        .take_while(|line| !line.is_empty())
        .collect::<Vec<_>>();
    println!("Request: {:#?}", http_request);

    let response = "HTTP/1.1 200 OK\r\n\r\n";
    stream.write_all(response.as_bytes())?;
    println!("Response: {:?}", response);
    Ok(())
}

fn main() -> std::io::Result<()> {
    let _myvec: Vec<u8> = Vec::new();
    let listener = TcpListener::bind("127.0.0.1:7878")?;

    listener.incoming().for_each(|stream| {
        println!("connecting at: {:#?}", stream);
        handle_connection(stream.unwrap()).unwrap()
    });
    // for stream in listener.incoming() {
    //     let stream = stream?;
    //     println!("Connection Established at: {:?}", stream);
    //     handle_connection(stream);
    // }
    Ok(())
}