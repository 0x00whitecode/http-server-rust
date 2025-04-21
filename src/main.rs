use std::net::TcpListener;
use std::io::{Read, Write};

fn main() {
    // Bind to localhost:4221
    let listener = TcpListener::bind("127.0.0.1:4221").unwrap();

    // Listen for incoming TCP connections
    for stream in listener.incoming() {
        match stream {
            Ok(mut stream) => {
                println!("Accepted new connection");

                // Read the HTTP request into a buffer
                let mut buffer = [0; 1024];
                let _ = stream.read(&mut buffer).unwrap();
                let request = String::from_utf8_lossy(&buffer);

                // Parse the request line
                if let Some(request_line) = request.lines().next() {
                    let parts: Vec<&str> = request_line.split_whitespace().collect();

                    if parts.len() >= 2 {
                        let method = parts[0];
                        let path = parts[1];

                        // Handle only GET requests
                        if method == "GET" {
                            // Handle root path "/"
                            if path == "/" {
                                let response = "HTTP/1.1 200 OK\r\n\r\n";
                                stream.write_all(response.as_bytes()).unwrap();
                                continue;
                            }

                            // Handle /echo/{str} path
                            if path.starts_with("/echo/") {
                                let echo_str = &path[6..]; // remove "/echo/"
                                let content_length = echo_str.len();

                                let response = format!(
                                    "HTTP/1.1 200 OK\r\nContent-Type: text/plain\r\nContent-Length: {}\r\n\r\n{}",
                                    content_length, echo_str
                                );
                                stream.write_all(response.as_bytes()).unwrap();
                                continue;
                            }

                            // Handle /user-agent
                            if path == "/user-agent" {
                                // Look for the User-Agent header
                                if let Some(user_agent_line) = request.lines()
                                    .find(|line| line.to_ascii_lowercase().starts_with("user-agent:"))
                                {
                                    let user_agent_value = user_agent_line.splitn(2, ":").nth(1).unwrap().trim();
                                    let content_length = user_agent_value.len();

                                    let response = format!(
                                        "HTTP/1.1 200 OK\r\nContent-Type: text/plain\r\nContent-Length: {}\r\n\r\n{}",
                                        content_length, user_agent_value
                                    );
                                    stream.write_all(response.as_bytes()).unwrap();
                                    continue;
                                }
                            }
                        }
                    }
                }

                // Default to 404 if no conditions matched
                let response = "HTTP/1.1 404 Not Found\r\n\r\n";
                stream.write_all(response.as_bytes()).unwrap();
            }
            Err(e) => {
                println!("Connection error: {}", e);
            }
        }
    }
}
