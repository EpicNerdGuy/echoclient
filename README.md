# echoclient

A minimal TCP client/server pair written in Rust, built while learning Rust networking fundamentals coming from a C background. The server basically echoes everything the client says.

## Overview

- **server.rs** listens on `127.0.0.1:6767`, accepts incoming TCP connections, and reads data sent by single connected client into a fixed 512-byte buffer.
- **client.rs** connects to the server and streams stdin input to it over the socket, also using a fixed 512-byte buffer.

No dynamic allocation for I/O buffers, reads and writes work directly against fixed-size stack arrays (`[u8; 512]`), similar to how you'd handle a raw socket buffer in C.

## Requirements

- Rust toolchain (`rustc`, and `cargo` if using the Cargo project structure)

## Build and run

### Using rustc directly

```bash
rustc server.rs
rustc client.rs
```

Run the server first, then the client, in separate terminals:

```bash
./server
```

```bash
./client
```

### Using Cargo (if set up as a workspace)

```bash
cargo run --bin server
cargo run --bin client
```

## Usage

1. Start the server. It prints `Server listening on port 6767` and waits for a connection.
2. Start the client. Once connected, type input and press Enter, it gets sent to the server over the socket.
3. The server prints whatever it receives.
4. Disconnect the client with Ctrl+D (EOF). The server logs the disconnect and returns to accepting new connections.

## Demo



https://github.com/user-attachments/assets/504efa2d-e829-4acc-8d68-e77eb35491e8



## Notes

- Currently handles one client connection at a time.
- Buffer size is fixed at 512 bytes per read.
