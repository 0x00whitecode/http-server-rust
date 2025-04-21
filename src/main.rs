use std::{env, fs, io::Write, net::TcpListener, path::{Path, PathBuf}, thread};
use std::io::{Read, Write as IoWrite};

fn handle_request(mut stream: std::net::TcpStream, request: String) {
    let mut resp = String::new();
    let not_found_resp = "HTTP/1.1 404 Not Found\r\n\r\n";

    // Check if the path starts with "/files"
    if request.starts_with("GET /files") {
        // Extract the file name from the request path
        let file_name = request.split_whitespace().nth(1).unwrap_or("");
        if file_name.starts_with("/files/") {
            let file_name = file_name.trim_start_matches("/files/");

            // Get the directory from the environment arguments
            let env_args: Vec<String> = env::args().collect();
            if env_args.len() < 3 {
                let error_msg = "Usage: ./your_program.sh --directory <directory_path>";
                resp = format!("HTTP/1.1 400 Bad Request\r\n\r\n{}", error_msg);
                stream.write_all(resp.as_bytes()).unwrap();
                return;
            }

            let mut dir = env_args[2].clone();
            dir.push_str(&file_name); // Append the file name to the directory path

            let path = Path::new(&dir);
            if path.exists() && path.is_file() {
                // Read the file content
                match fs::read(path) {
                    Ok(fc) => {
                        // Serve the file content as binary (don't convert to string)
                        resp = format!(
                            "HTTP/1.1 200 OK\r\nContent-Type: application/octet-stream\r\nContent-Length: {}\r\n\r\n",
                            fc.len()
                        );
                        // Write the response header first
                        stream.write_all(resp.as_bytes()).unwrap();
                        // Then send the file contents
                        stream.write_all(&fc).unwrap();
                    }
                    Err(_) => {
                        resp = not_found_resp.to_string();
                        stream.write_all(resp.as_bytes()).unwrap();
                    }
                }
            } else {
                // If file doesn't exist
                resp = not_found_resp.to_string();
                stream.write_all(resp.as_bytes()).unwrap();
            }
        }
    } else {
        // Handle non-GET requests or other errors
        resp = not_found_resp.to_string();
        stream.write_all(resp.as_bytes()).unwrap();
    }
}

fn main() {
    // Get command line arguments
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 || args[1] != "--directory" {
        eprintln!("Usage: ./your_program.sh --directory <directory_path>");
        return;
    }
    let directory = &args[2];

    // Start the server
    let listener = TcpListener::bind("127.0.0.1:4221").unwrap();
    println!("Server listening on 127.0.0.1:4221...");

    for stream in listener.incoming() {
        match stream {
            Ok(mut stream) => {
                let mut buffer = [0; 1024];
                let _ = stream.read(&mut buffer).unwrap();
                let request = String::from_utf8_lossy(&buffer);
                // Handle each request in a separate thread
                let directory_clone = directory.to_string();
                thread::spawn(move || {
                    handle_request(stream, request.to_string());
                });
            }
            Err(_) => {
                eprintln!("Error accepting connection.");
            }
        }
    }
}
