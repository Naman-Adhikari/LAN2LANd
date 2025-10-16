use std::fs::File;
use std::io::BufRead;
use std::io::{self, Read, Write};
use std::net::{TcpListener, TcpStream, UdpSocket};
use std::path::Path;
use std::process::Command;
use std::thread;
use std::time::Duration;
use hostname;

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
    println!("Scanning for peers in your LAN....");

    // Discover peers via UDP
    let peers = discover_peers();
    if peers.is_empty() {
        println!("No peers found!");
        return; 
    }

    // Show peers if available
    println!("Available Peers: ");
    for(i, (name, ip)) in peers.iter().enumerate() {
        println!("[{}] {} ({})", i+1, name, ip);
    }

    print!("Select peer to send file: ");
    io::stdout().flush().unwrap();
    let mut choice = String::new();
    io::stdin().read_line(&mut choice).unwrap();

    let choice : usize = choice.trim().parse().unwrap();

    let (_, peer_ip) = &peers[choice - 1];

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

    match TcpStream::connect(format!("{}:7878", peer_ip)) {
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

    thread::spawn(|| {
        listen_for_discovery();
    })

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

fn discover_peers() -> Vec<(String, String)> {
    let broadcast_addr = "255.255.255.255:9999";
    let local_socket = UdpSocket::bind("0.0.0.0:0").expect("Couldnt bind UDP Socket");

    //allow broadcast
    local_socket.set_broadcast(true).unwrap();

    // get hostname
    let hostname = hostname::get().unwrap().to_string_lossy().to_string();


    let msg = format!("{} : {}", hostname, 0);
    local_socket
        .send_to(msg.as_bytes(), broadcast_addr)
        .unwrap();

    // listen for 1 sec for replies
    local_socket
        .set_read_timeout(Some(Duration::from_secs(1)))
        .unwrap();

    let mut peers = Vec::new();
    let mut buf = [0u8; 128];

    while let Ok((size, src)) = local_socket.recv_from(&mut buf) {
            let received = String::from_utf8_lossy(&buf[..size]);
            let name = received.split(':').next().unwrap_or("Unknown").to_string();
            let ip = src.ip().to_string();

            peers.push((name, ip));
        }
    peers
}

fn listen_for_discovery() {
    let socket = UdpSocket::bind("0.0.0.0:9999").expect("Could not bind UDP discovery");

    let hostname = hostname::get().unwrap().to_string_lossy().to_string();

    let mut buf = [0u8; 128];

    println!("Listening for LAN discovery on UDP port 9999");

    loop {
        if let Ok((size, src)) = socket.recv_from(&mut buf) {
            let msg = String::from_utf8_lossy(&buf[..size]);
            if msg == "DISCOVER" {
                socket.send_to(hostname.as_bytes(), src).unwrap();
                println!("Responded to discovery request from {}", src);
            }
        }
    }
}