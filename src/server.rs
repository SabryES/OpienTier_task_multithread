use crate::message::EchoMessage; // Import EchoMessage
use log::{error, info, warn}; // Import logging macros
use prost::Message; // Import prost for message encoding/decoding
use std::{
    io::{self, ErrorKind, Read, Write}, // Import I/O traits and modules
    net::{TcpListener, TcpStream}, // Import networking modules
    sync::{
        atomic::{AtomicBool, Ordering}, // Import atomic types for thread-safe shared state
        Arc, // Import Arc for atomic reference counting
    },
    thread, // Import threading module
    time::Duration, // Import Duration for handling timeouts
};

struct Client {
    stream: TcpStream, // TCP stream for the client
}

impl Client {
    pub fn new(stream: TcpStream) -> Self {
        Client { stream } // Create a new client with the given stream
    }

    pub fn handle(&mut self, timeout: Duration) -> io::Result<()> {
        self.stream.set_read_timeout(Some(timeout))?; // Set read timeout for the stream
        let mut buffer = [0; 512]; // Create a buffer
        // Read data from the client
        loop {
            let bytes_read = self.stream.read(&mut buffer)?; // Read from stream
            if bytes_read == 0 {
                info!("Client disconnected."); // Log client disconnection
                return Ok(()); // Return Ok if client disconnected
            }

            if let Ok(message) = EchoMessage::decode(&buffer[..bytes_read]) {
                info!("Received: {}", message.content); // Log received message
                // Echo back the message
                let payload = message.encode_to_vec(); // Encode message to vector
                self.stream.write_all(&payload)?; // Write payload to stream
                self.stream.flush()?; // Flush the stream
            } else {
                error!("Failed to decode message"); // Log decoding error
            }
        }
    }
}

pub struct Server {
    listener: TcpListener, // TCP listener for the server
    is_running: Arc<AtomicBool>, // Atomic flag to indicate if the server is running
}

impl Server {
    /// Creates a new server instance
    pub fn new(addr: &str) -> io::Result<Self> {
        let listener = TcpListener::bind(addr)?; // Bind the listener to the address
        let is_running = Arc::new(AtomicBool::new(false)); // Initialize the running flag
        Ok(Server {
            listener, // Set the listener
            is_running, // Set the running flag
        })
    }

    /// Runs the server, listening for incoming connections and handling them
    pub fn run(&self) -> io::Result<()> {
        self.is_running.store(true, Ordering::SeqCst); // Set the server as running
        info!("Server is running on {}", self.listener.local_addr()?); // Log server running

        // Set the listener to non-blocking mode
        self.listener.set_nonblocking(true)?; // Set listener to non-blocking

        while self.is_running.load(Ordering::SeqCst) {
            match self.listener.accept() {
                Ok((stream, addr)) => {
                    info!("New client connected: {}", addr); // Log new client connection

                    // Handle the client request in a new thread
                    let mut client = Client::new(stream); // Create new client
                    let timeout = Duration::from_secs(5); // Set a timeout duration
                    thread::spawn(move || {
                        if let Err(e) = client.handle(timeout) {
                            error!("Error handling client: {}", e); // Log error handling client
                        }
                    });
                }
                Err(ref e) if e.kind() == ErrorKind::WouldBlock => {
                    // No incoming connections, sleep briefly to reduce CPU usage
                    thread::sleep(Duration::from_millis(100)); // Sleep to reduce CPU usage
                }
                Err(e) => {
                    error!("Error accepting connection: {}", e); // Log error accepting connection
                }
            }
        }

        info!("Server stopped."); // Log server stopped
        Ok(())
    }

    /// Stops the server by setting the `is_running` flag to `false`
    pub fn stop(&self) {
        if self.is_running.load(Ordering::SeqCst) {
            self.is_running.store(false, Ordering::SeqCst); // Set running flag to false
            info!("Shutdown signal sent."); // Log shutdown signal
        } else {
            warn!("Server was already stopped or not running."); // Log server already stopped
        }
    }
}