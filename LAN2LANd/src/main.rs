use std::fs::File;
use std::io::BufRead;
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

fn receive_mode() {
    let listener = TcpListener::bind("0.0.0.0:7878").expect("Could not bind to port 7878");

    println!("Listening on port 7878.....");
    println!("Waiting for sender.......");

    for stream in listener.incoming() {
        match stream {
            Ok(mut stream) => {
                println!("Connected: {}", stream.peer_addr().unwrap());
                thread::spawn(move || {
                    handle_connection(&mut stream);
                });
            }
            Err(e) => eprintln!("Connection error {}", e),
        }
    }
}

fn handle_connection(stream: &mut TcpStream) {
    let mut reader = io::BufReader::new(stream);
    let mut filename = String::new();
    // read filename line
    reader.read_line(&mut filename).unwrap();
    let filename = filename.trim();

    let mut outfile = File::create(filename).unwrap();
    let mut buffer = [0u8; 4096];
    loop {
        let n = reader.read(&mut buffer).unwrap_or(0);
        if n == 0 {
            break;
        }
        outfile.write_all(&buffer[..n]).unwrap();
    }
    println!("Received file: {}", filename);
    println!("Enter the full file path to send: ");
    let mut path = String::new();
    io::stdin().read_line(&mut path).unwrap();
    let path = path.trim();

    println!("Enter receiver IP: ");
    io::stdout().flush().unwrap();
    let mut ip = String::new();
    io::stdin().read_line(&mut ip).unwrap();
    let ip = ip.trim();

    match TcpStream::connect(format!("{}:7878", ip)) {
        Ok(mut stream) => {
            let path_obj = Path::new(path);
            let file_name = path_obj.file_name().unwrap().to_string_lossy();
            let mut file = File::open(path_obj).expect("failed to open file");

            stream
                .write_all(format!("{}\n", file_name).as_bytes())
                .unwrap();

            let mut buffer = [0u8; 4096];
            loop {
                let n = file.read(&mut buffer).unwrap();
                if n == 0 {
                    break;
                }
                stream.write_all(&buffer[..n]).unwrap();
            }
            println!("File sent successfully!!");
        }
        Err(e) => eprintln!("Connection failed: {}", e),
    }
}
