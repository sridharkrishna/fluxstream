// src/main.rs

// Copyright 2024 Kriyaetive Verse Private Limited
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.
//
// Author: Sridhar Ananthakrishnan <itsmycodehub@gmail.com>

use std::net::{TcpListener, TcpStream};
use std::io::{Read, Write};
use std::thread;

/// Handles each incoming connection.
fn handle_client(mut stream: TcpStream) {
    let mut buffer = [0; 512];  // Buffer to store incoming data

    loop {
        // Read data from the stream
        match stream.read(&mut buffer) {
            Ok(0) => {
                // If read returns 0, it means the connection was closed
                println!("Connection closed by client.");
                break;
            },
            Ok(bytes_read) => {
                println!("Received: {}", String::from_utf8_lossy(&buffer[..bytes_read]));

                // Echo the data back to the client
                if let Err(e) = stream.write(&buffer[..bytes_read]) {
                    eprintln!("Failed to send response: {}", e);
                    break;
                }
            },
            Err(e) => {
                eprintln!("Failed to read from connection: {}", e);
                break;
            }
        }
    }
}

fn main() -> std::io::Result<()> {
    // Set up a TCP listener on port 7878
    let listener = TcpListener::bind("127.0.0.1:7878")?;
    println!("Server listening on port 7878");

    // Accept incoming connections in a loop
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                println!("New connection established!");

                // Spawn a new thread to handle each client connection
                thread::spawn(move || {
                    handle_client(stream);
                });
            },
            Err(e) => {
                eprintln!("Failed to establish connection: {}", e);
            }
        }
    }
    Ok(())
}


