use std::fs::File;
use std::io::{self, Read, Write};
use std::net::{TcpListener, TcpStream};
use std::path::Path;
use std::thread;

fn main() {
    println!("Simple LAN File Transfer");
    println!("1. Send file");
    println!("2. Receive file");
    print!("Choose (1/2): ");
    io::stdout().flush().unwrap();

    let mut choice = String::new();
    io::stdin().read_line(&mut choice).unwrap();

    match choice.trim() {
        "1" => send_mode(),
        "2" => receive_mode(),
        _ => println!("Invalid choice"),
    }
}

fn send_mode() {
    print!("Enter receiver IP (e.g. 192.168.1.5): ");
    io::stdout().flush().unwrap();
    let mut ip = String::new();
    io::stdin().read_line(&mut ip).unwrap();
    let ip = ip.trim();

    print!("Enter file path to send: ");
    io::stdout().flush().unwrap();
    let mut path = String::new();
    io::stdin().read_line(&mut path).unwrap();
    let path = path.trim();

    match TcpStream::connect(format!("{}:7878", ip)) {
        Ok(mut stream) => {
            let path_obj = Path::new(path);
            let file_name = path_obj.file_name().unwrap().to_string_lossy();
            let mut file = File::open(path_obj).expect("Failed to open file");

            // send filename first (terminated by newline)
            stream
                .write_all(format!("{}\n", file_name).as_bytes())
                .unwrap();

            // send file bytes
            let mut buffer = [0u8; 4096];
            loop {
                let n = file.read(&mut buffer).unwrap();
                if n == 0 {
                    break;
                }
                stream.write_all(&buffer[..n]).unwrap();
            }
            println!("File sent successfully!");
        }
        Err(e) => eprintln!("Connection failed: {}", e),
    }
}
