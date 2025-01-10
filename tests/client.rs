use embedded_recruitment_task::message::{client_message, ServerMessage}; // Import necessary modules
use log::error; // Import logging for errors
use log::info; // Import logging for info
use prost::Message; // Import prost for message encoding/decoding
use std::io::Read; // Import Read trait for reading from streams
use std::io::Write; // Import Write trait for writing to streams
use std::{
    io, // Import io module for handling I/O operations
    net::{SocketAddr, TcpStream, ToSocketAddrs}, // Import networking modules
    time::Duration, // Import Duration for handling timeouts
};

// TCP/IP Client
pub struct Client {
    ip: String, // IP address of the server
    port: u32, // Port number of the server
    timeout: Duration, // Connection timeout duration
    stream: Option<TcpStream>, // Optional TCP stream for the connection
}

impl Client {
    pub fn new(ip: &str, port: u32, timeout_ms: u64) -> Self {
        Client {
            ip: ip.to_string(), // Convert IP to string
            port, // Set port
            timeout: Duration::from_millis(timeout_ms), // Set timeout duration
            stream: None, // Initialize stream as None
        }
    }

    // Connect the client to the server
    pub fn connect(&mut self) -> io::Result<()> {
        println!("Connecting to {}:{}", self.ip, self.port); // Print connection message

        // Resolve the address
        let address = format!("{}:{}", self.ip, self.port); // Format address
        let socket_addrs: Vec<SocketAddr> = address.to_socket_addrs()?.collect(); // Resolve address to socket addresses

        if socket_addrs.is_empty() {
            return Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                "Invalid IP or port",
            )); // Return error if no valid addresses
        }

        // Connect to the server with a timeout
        let stream = TcpStream::connect_timeout(&socket_addrs[0], self.timeout)?; // Connect to the server
        self.stream = Some(stream); // Set the stream

        println!("Connected to the server!"); // Print success message
        Ok(())
    }

    // Disconnect the client
    pub fn disconnect(&mut self) -> io::Result<()> {
        if let Some(stream) = self.stream.take() {
            stream.shutdown(std::net::Shutdown::Both)?; // Shutdown the stream
        }

        println!("Disconnected from the server!"); // Print disconnection message
        Ok(())
    }

    // Send a message to the server
    pub fn send(&mut self, message: client_message::Message) -> io::Result<()> {
        if let Some(ref mut stream) = self.stream {
            // Encode the message to a buffer
            let mut buffer = Vec::new(); // Create a buffer
            if let Err(e) = message.encode(&mut buffer) {
                return Err(io::Error::new(io::ErrorKind::InvalidData, e.to_string()));
            }

            // Send the buffer to the server
            stream.write_all(&buffer)?; // Write buffer to stream
            stream.flush()?; // Flush the stream

            println!("Sent message: {:?}", message); // Print sent message
            Ok(())
        } else {
            Err(io::Error::new(
                io::ErrorKind::NotConnected,
                "No active connection",
            )) // Return error if no active connection
        }
    }

    // Receive a message from the server
    pub fn receive(&mut self) -> io::Result<ServerMessage> {
        if let Some(ref mut stream) = self.stream {
            info!("Receiving message from the server"); // Log receiving message
            let mut buffer = vec![0u8; 1024]; // Create a buffer
            let bytes_read = stream.read(&mut buffer)?; // Read from stream
            if bytes_read == 0 {
                info!("Server disconnected."); // Log server disconnection
                return Err(io::Error::new(
                    io::ErrorKind::ConnectionAborted,
                    "Server disconnected",
                )); // Return error if server disconnected
            }

            info!("Received {} bytes from the server", bytes_read); // Log received bytes

            // Decode the received message
            ServerMessage::decode(&buffer[..bytes_read]).map_err(|e| {
                io::Error::new(
                    io::ErrorKind::InvalidData,
                    format!("Failed to decode ServerMessage: {}", e),
                ) // Return error if decoding fails
            })
        } else {
            error!("No active connection"); // Log no active connection
            Err(io::Error::new(
                io::ErrorKind::NotConnected,
                "No active connection",
            )) // Return error if no active connection
        }
    }
}