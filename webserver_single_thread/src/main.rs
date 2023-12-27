use std::{
    fs,
    io::{BufRead, BufReader, Write},
    net::{TcpListener, TcpStream},
    thread,
    time::Duration,
};

use webserver_single_thread::thread_pool::ThreadPool;

fn handle_connection(mut stream: TcpStream) -> std::io::Result<()> {
    let buf_reader = BufReader::new(&mut stream);
    let request_line = buf_reader.lines().next().unwrap()?;

    let (status_line, filename) = match &request_line[..] {
        "GET / HTTP/1.1" => {
            println!("thread id:{:#?}", thread::current().id());
            ("HTTP/1.1 200 OK", "hello.html")
        }
        "GET /sleep HTTP/1.1" => {
            println!("thread id sleep: {:#?}", thread::current().id());
            thread::sleep(Duration::from_secs(5));
            ("HTTP/1.1 200 OK", "hello.html")
        }
        _ => ("HTTP/1.1 404 NOT FOUND", "404.html"),
    };

    let contents = fs::read_to_string(filename)?;
    let length = contents.len();
    let response = format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");
    stream.write_all(response.as_bytes())?;
    println!("Response: {:?}", response);

    // let http_request = buf_reader
    //     .lines()
    //     .map(|result| result.unwrap())
    //     .take_while(|line| !line.is_empty())
    //     .collect::<Vec<_>>();
    // println!("Request: {:#?}", http_request);

    // let status_line = "HTTP/1.1 200 OK";
    // let content = fs::read_to_string("hello.html")?;
    // let length = content.len();

    // let response = format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{content}");

    // stream.write_all(response.as_bytes())?;
    // println!("Response: {:?}", response);
    Ok(())
}

fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:7878")?;
    let pool = ThreadPool::new(4);

    // after 2 incoming requests, the Threadpool will shutdown
    // using Drop implementation
    listener.incoming().take(2).for_each(|stream| {
        pool.execute(|| handle_connection(stream.unwrap()).unwrap());
        // println!("connecting at: {:#?}", stream);
        // std::thread::spawn(|| handle_connection(stream.unwrap()).unwrap());
    });
    // for stream in listener.incoming() {
    //     let stream = stream?;
    //     println!("Connection Established at: {:?}", stream);
    //     handle_connection(stream)?;
    // }

    Ok(())
}
