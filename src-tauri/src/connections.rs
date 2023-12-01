use std::{net::TcpStream, io::{Write, Read}};

#[tauri::command]
pub fn start_transmission_server() {

    let mut stream = TcpStream::connect("localhost:8080").expect("Error connecting to server");

    let data_to_send = "Hello World from Transmitter!";
    stream.write_all(data_to_send.as_bytes()).expect("Failed to write buffer to stream");

}

#[tauri::command]
pub fn start_reception_server() {

    let mut buffer = [0; 1024]; // Buffer to store incoming data

    // Read data from the stream
    match stream.read(&mut buffer) {
        Ok(size) => {
            // Print the received message
            println!("Received message: {:?}", &buffer[..size]);

            // You can process or respond to the message here
        }
        Err(e) => {
            eprintln!("Error reading from stream: {}", e);
        }
    }


}