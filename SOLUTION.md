# Solution
Bugs Identified and Resolved:
1. protoc Not Found
                    Issue: The protoc binary was not found, causing the build script to fail. 
                    Solution: Ensure protoc is installed and available in the system's PATH. Updated the build.rs to check for protoc using the which crate.

2. Incorrect Usage of which Crate
                                Issue: The which crate was not imported correctly in build.rs. 
                                Solution: Added use which::which; to import the which crate correctly.

3. Incorrect Handling of prost::Message::encode
                                                Issue: The encode method was incorrectly assumed to return a Result, causing mismatched types.
                                                 Solution: Updated the encode method usage to handle errors directly without using map_err.

4. Incorrect Enum Variant Usage
                                Issue: The client_message::Message enum was incorrectly used as a struct. 
                                Solution: Corrected the usage to match the enum variant structure.

5. Missing Imports
                    Issue: Missing imports for EchoMessage and server_message. 
                    Solution: Added the necessary imports to ensure the types are correctly recognized.

6. Incorrect Field Access
                        Issue: Attempted to access a non-existent field content in ServerMessage. 
                        Solution: Corrected the field access to match the actual structure of ServerMessage.

Changes Made to Convert to Multi-threaded Server
Original Single-threaded Version
                                In the original single-threaded version, the server handled one client at a time, blocking other clients. The run method continuously accepted new connections and handled them in the same thread.

Multi-threaded Version
                        To handle multiple clients concurrently, the server was updated to use threads. Each client connection is now handled in a separate thread using thread::spawn.

Code Changes
Client Struct: Represents a client connection.

new: Initializes a new client with a TcpStream.
handle: Reads data from the client, decodes it, and echoes it back.
Server Struct: Represents the server.

new: Creates a new server instance, binding to the specified address.
run: Runs the server, accepting and handling client connections in separate threads.
stop: Stops the server by setting the is_running flag to false.

Design Flaws Addressed

1. Single-threaded Handling
Flaw: The original design handled one client at a time, blocking other clients. 
Solution: Updated the server to handle multiple clients concurrently using threads.

2. Error Handling
Flaw: Some error handling was missing or incorrect. 
Solution: Improved error handling and logging throughout the code.

3. Graceful Shutdown
Flaw: The server did not handle shutdown signals gracefully. 
Solution: Added a stop method to set the is_running flag to false, allowing the server to shut down gracefully.

4. Configuration
Flaw: The server address was hardcoded. 
Solution: Allowed configuration of server parameters such as address and port.

This document provides a comprehensive overview of the bugs identified and resolved, the changes made to convert the server to a multi-threaded design, and the design flaws addressed during the update.