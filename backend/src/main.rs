use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use mime_guess::from_path;
use include_dir::{include_dir, Dir};

static DIST_DIR: Dir = include_dir!("$CARGO_MANIFEST_DIR/../frontend/dist");

fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:8686")?;
    println!("Server running on http://127.0.0.1:8686");

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => handle_connection(stream),
            Err(e) => eprintln!("Connection failed: {}", e),
        }
    }

    Ok(())
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 1024];
    if stream.read(&mut buffer).is_ok() {
        let request = String::from_utf8_lossy(&buffer);
        let first_line = request.lines().next().unwrap_or("");
        let path = first_line.split_whitespace().nth(1).unwrap_or("/");

        let file_path = if path == "/" {
            "index.html"
        } else {
            &path[1..] // Strip leading '/'
        };

        match DIST_DIR.get_file(file_path) {
            Some(file) => {
                let contents = file.contents();
                let content_type = from_path(file.path()).first_or_octet_stream();
                let response = format!(
                    "HTTP/1.1 200 OK\r\nContent-Type: {content_type}\r\n\r\n",
                    content_type = content_type
                );
                stream.write_all(response.as_bytes()).unwrap();
                stream.write_all(contents).unwrap();
            }
            None => send_404(&mut stream),
        }
    }
}

fn send_404(stream: &mut TcpStream) {
    let response = "HTTP/1.1 404 NOT FOUND\r\n\r\n404 Not Found";
    stream.write_all(response.as_bytes()).unwrap();
}
