use multi::ThreadPool;
use std::{
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream}, fs, thread,time::Duration,
};
fn main(){
    let listener = TcpListener::bind("127.0.0.1:7878").expect("hi2");
    let pool = ThreadPool::new(4);

    for stream in listener.incoming(){
        let stream = stream.expect("hi3");
        pool.execute(|| {
            handle_connection(stream);
        });
    }
}

fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&mut stream);
    let request_line = buf_reader.lines().next().expect("hi4").expect("ji5");

    let (status_line, filename) = match &request_line[..] {
        "GET / HTTP/1.1" => ("HTTP/1.1 200 OK", "hello.html"),
        "GET /sleep HTTP/1.1" => {
            thread::sleep(Duration::from_secs(5));
            ("HTTP/1.1 200 OK", "hello.html")
        }
        _ => ("HTTP/1.1 404 NOT FOUND", "404.html"),
    };

    let contents = fs::read_to_string(filename).expect("hi6");
    let length = contents.len();

    let response =
        format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");

    stream.write_all(response.as_bytes()).expect("hi7");
        
}
