use std::fs::File;
use std::io::BufRead;
use std::io::{self, Read, Write};
use std::net::{TcpListener, TcpStream};
use std::path::Path;
use std::process::Command;
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
    let file_path = match Command::new("zenity")
        .arg("--file-selection")
        .arg("--title=Select a file to send")
        .output()
    {
        Ok(output) if output.status.success() => {
            String::from_utf8_lossy(&output.stdout).trim().to_string()
        }
        _ => {
            println!("No files selected! ABORTING...");
            return;
        }
    };
    println!("selected file: {}", file_path);

    println!("Enter receiver IP : 192.168.1.");
    io::stdout().flush().unwrap();
    let mut ip = String::new();
    io::stdin().read_line(&mut ip).unwrap();
    let ip = ip.trim();

    match TcpStream::connect(format!("192.168.1.{}:7878", ip)) {
        Ok(mut stream) => {
            let path_obj = Path::new(&file_path);
            let file_name = path_obj.file_name().unwrap().to_string_lossy();
            let mut file = File::open(path_obj).expect("Failed to open file");

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
            println!("File sent successfully!");
        }
        Err(e) => eprintln!("Connection failed: {}", e),
    }
}

fn receive_mode() {
    let listener = TcpListener::bind("0.0.0.0:7878").expect("Could not bind");
    println!("Listening on port 7878...");
    println!("Waiting for sender...");

    for stream in listener.incoming() {
        match stream {
            Ok(mut stream) => {
                println!("Connected: {}", stream.peer_addr().unwrap());
                thread::spawn(move || {
                    handle_connection(&mut stream);
                });
            }
            Err(e) => eprintln!("Connection error: {}", e),
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
}
