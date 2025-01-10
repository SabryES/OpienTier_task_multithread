use embedded_recruitment_task::{
    message::{self, client_message, EchoMessage, ServerMessage, server_message}, // Import necessary modules
    server::Server, // Import server module
};
use std::{
    sync::Arc, // Import Arc for atomic reference counting
    thread::{self, JoinHandle}, // Import threading modules
};

mod client; // Import client module

fn setup_server_thread(server: Arc<Server>) -> JoinHandle<()> {
    thread::spawn(move || {
        server.run().expect("Server encountered an error"); // Run the server in a new thread
    })
}

fn create_server() -> Arc<Server> {
    Arc::new(Server::new("localhost:8080").expect("Failed to start server")) // Create a new server instance
}

#[test]
fn test_client_connection() {
    // Set up the server in a separate thread
    let server = create_server(); // Create server
    let handle = setup_server_thread(server.clone()); // Set up server thread

    // Create and connect the client
    let mut client = client::Client::new("localhost", 8080, 1000); // Create client
    client.connect().expect("Failed to connect to server"); // Connect client

    // Disconnect the client
    client.disconnect().expect("Failed to disconnect from server"); // Disconnect client

    // Stop the server
    server.stop(); // Stop server
    handle.join().expect("Failed to join server thread"); // Join server thread
}

#[test]
fn test_client_echo_message() {
    // Set up the server in a separate thread
    let server = create_server(); // Create server
    let handle = setup_server_thread(server.clone()); // Set up server thread

    // Create and connect the client
    let mut client = client::Client::new("localhost", 8080, 1000); // Create client
    client.connect().expect("Failed to connect to server"); // Connect client

    // Send an echo message
    let message = client_message::Message::EchoMessage(EchoMessage {
        content: "Hello, server!".to_string(),
    });
    client.send(message).expect("Failed to send message"); // Send message

    // Receive the echo message
    let response: ServerMessage = client.receive().expect("Failed to receive message"); // Receive message
    if let Some(server_message::Message::EchoMessage(echo_message)) = response.message {
        assert_eq!(echo_message.content, "Hello, server!"); // Assert received message
    } else {
        panic!("Unexpected message type"); // Panic if unexpected message type
    }

    // Disconnect the client
    client.disconnect().expect("Failed to disconnect from server"); // Disconnect client

    // Stop the server
    server.stop(); // Stop server
    handle.join().expect("Failed to join server thread"); // Join server thread
}