use std::net::TcpListener;
use std::io::{Read, Write}; // Add Read to read from the stream

fn main() {
    let listener = TcpListener::bind("127.0.0.1:4221").unwrap();

    for stream in listener.incoming() {
        match stream {
            Ok(mut stream) => {
                println!("accepted new connection");

                let mut buffer = [0; 1024];
                let _ = stream.read(&mut buffer).unwrap();
                let request = String::from_utf8_lossy(&buffer);

                // Split the request into lines and parse the request line
                if let Some(request_line) = request.lines().next() {
                    let parts: Vec<&str> = request_line.split_whitespace().collect();
                    if parts.len() >= 2 {
                        let method = parts[0];
                        let path = parts[1];

                        // Handle only GET requests
                        if method == "GET" {
                            if path == "/" {
                                let response = "HTTP/1.1 200 OK\r\n\r\n";
                                stream.write_all(response.as_bytes()).unwrap();
                                continue;
                            }
                        }
                    }
                }

                // Default response for all other cases
                let response = "HTTP/1.1 404 Not Found\r\n\r\n";
                stream.write_all(response.as_bytes()).unwrap();
            }
            Err(e) => {
                println!("error: {}", e);
            }
        }
    }
}
