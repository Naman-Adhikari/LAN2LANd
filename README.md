# 🖧 LAN2LANd

**LAN2LANd** is a simple, cross-platform **LAN file transfer utility** written in **Rust**.  
It allows two systems on the same network to quickly send and receive files without any external dependencies — just Rust and a local connection.

---

## 🚀 Features

- 📂 Send and receive files over LAN  
- ⚡ Fast transfer via TCP sockets  
- 🪟 Uses **Zenity** for graphical file selection on Linux  
- 💬 Simple interactive CLI  
- 🧵 Multithreaded receiver — handles multiple incoming transfers  

---

## 🧰 Requirements

- 🦀 [Rust](https://www.rust-lang.org/tools/install) (latest stable version)
- 🪟 **Zenity** (for Linux GUI file picker)  
  Install via:
  ```bash
  sudo apt install zenity
 ```

 🛠️ Build & Run

Clone the repository:

```bash
git clone git@github.com:Itsjustme27/LAN2LANd.git
cd LAN2LANd
```

Build the project:

```bash
cargo build --release
```

Run the executable:

```bash
cargo run
```

### 💡 Usage
#### ▶️ Sender

Choose “Send file” when prompted.

Select the file using the Zenity dialog.

Enter the receiver’s last octet (e.g., 105 for 192.168.1.105).

Wait for “File sent successfully!”

#### 💾 Receiver

Choose “Receive file” mode.

The program will listen on port 7878.

When a sender connects, the file will automatically be saved to the current directory.

## 🌐 Example

### On Receiver (Machine B):
```bash
$ cargo run
Simple LAN File Transfer
1. Send file
2. Receive file
Choose (1/2): 2
Listening on port 7878...
Waiting for sender...
Connected: 192.168.1.104:52374
Received file: report.pdf
```

### On Sender (Machine A) :

```bash
$ cargo run
Simple LAN File Transfer
1. Send file
2. Receive file
Choose (1/2): 1
selected file: /home/user/report.pdf
Enter receiver IP : 192.168.1.
105
File sent successfully!
```

⚠️ Notes

Ensure both devices are on the same subnet (e.g., 192.168.1.x).

Port 7878 must be open on the receiver.

Currently supports one file at a time.

For large files, use wired LAN for better stability.

🧩 Future Plans

🔒 Add checksum verification

🖼️ Cross-platform GUI version

📁 Multi-file batch transfer

🌍 Automatic IP discovery on LAN

### 👨‍💻 Author

Itsjustme27
Naman-Adhikari

Built with ❤️ in Rust.

