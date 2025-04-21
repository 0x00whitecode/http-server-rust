use std::env;
use std::fs::File;
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::path::{Path, PathBuf};
use std::thread;

fn handle_request(mut stream: TcpStream, request: String, directory: String) {
    // Parse the HTTP request
    if let Some(request_line) = request.lines().next() {
        let parts: Vec<&str> = request_line.split_whitespace().collect();
        if parts.len() >= 2 {
            let method = parts[0];
            let path = parts[1];

            if method == "GET" {
                // Handle /files/{filename} path
                if path.starts_with("/files/") {
                    let filename = &path[7..]; // Extract filename from /files/{filename}

                    let file_path = format!("{}/{}", directory, filename); // Build the full path

                    // Check if the file exists in the directory
                    if Path::new(&file_path).exists() {
                        // File exists, send it back
                        let mut file = File::open(file_path).unwrap();
                        let mut contents = Vec::new();
                        file.read_to_end(&mut contents).unwrap();
                        let content_length = contents.len();

                        // Response header
                        let response_header = format!(
                            "HTTP/1.1 200 OK\r\nContent-Type: application/octet-stream\r\nContent-Length: {}\r\n\r\n",
                            content_length
                        );

                        // Write header and file content
                        stream.write_all(response_header.as_bytes()).unwrap();
                        stream.write_all(&contents).unwrap();
                        return;
                    } else {
                        // File does not exist, send 404
                        let response = "HTTP/1.1 404 Not Found\r\n\r\n";
                        stream.write_all(response.as_bytes()).unwrap();
                        return;
                    }
                }
            }
        }
    }

    // Default to 404 if no conditions matched
    let response = "HTTP/1.1 404 Not Found\r\n\r\n";
    stream.write_all(response.as_bytes()).unwrap();
}

fn main() {
    // Parse command line arguments
    let args: Vec<String> = env::args().collect();
    let directory = match args.get(1) {
        Some(arg) if arg == "--directory" => args.get(2).expect("Directory path missing"),
        _ => {
            eprintln!("Usage: ./your_program.sh --directory <directory_path>");
            return;
        }
    };

    // Bind to localhost:4221
    let listener = TcpListener::bind("127.0.0.1:4221").unwrap();

    // Listen for incoming TCP connections
    for stream in listener.incoming() {
        match stream {
            Ok(mut stream) => {
                println!("Accepted new connection");

                // Handle each connection in a separate thread
                thread::spawn({
                    let directory = directory.clone(); // Clone the directory string
                    move || {
                        let mut buffer = [0; 1024];
                        let _ = stream.read(&mut buffer).unwrap();
                        let request = String::from_utf8_lossy(&buffer);

                        handle_request(stream, request.to_string(), directory);
                    }
                });
            }
            Err(e) => {
                println!("Connection error: {}", e);
            }
        }
    }
}
