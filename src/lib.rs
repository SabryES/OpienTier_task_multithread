pub mod server; // Export server module

pub mod message {
    include!(concat!(env!("OUT_DIR"), "/messages.rs")); // Include generated protobuf messages
}