use std::fs::File;
use std::io::{self, Read, Write};
use std::net::{TcpListener, TcpStream};
use std::path::Path;
use std::thread;

fn main() {
println!("Simple LAN-2-LAN transfer");
println!("1. Send file ");
println!("1. Recieve file ");
println!("Choose 1 or 2!");
io::stdout().flush().unwrap();

let mut choice = String::new();
io::stdin().read_line(&mut choice).unwrap();

 
