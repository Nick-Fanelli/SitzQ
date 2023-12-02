use tokio::net::{TcpListener, TcpStream};
use tokio::io::{AsyncReadExt, AsyncWriteExt};

pub struct NetworkRemote {

    pub uuid: String,
    address: String,
    stream: Option<TcpStream>

}

impl NetworkRemote {

    pub fn new(uuid: String, address: String) -> Self {

        NetworkRemote { 
            uuid,
            address,
            stream: None
        }

    }

    pub async fn connect(&mut self) -> Result<(), std::io::Error> {
        let stream = TcpStream::connect(&self.address).await?;
        self.stream = Some(stream);
        Ok(())
    }

    pub async fn send_message(&mut self, message: &str) -> Result<(), std::io::Error> {
        if let Some(ref mut stream) = self.stream {
            stream.write_all(message.as_bytes()).await?;
            Ok(())
        } else {
            Err(std::io::Error::new(
                std::io::ErrorKind::NotConnected,
                "Stream not connected"
            ))
        }
    }

    pub async fn receive_message(&mut self) -> Result<String, std::io::Error> {

        if let Some(ref mut stream) = self.stream {
            let mut buffer = [0; 1024];
            let bytes_read = stream.read(&mut buffer).await?;
            Ok(String::from_utf8_lossy(&buffer[..bytes_read]).to_string())
        } else {
            Err(std::io::Error::new(
                std::io::ErrorKind::NotConnected,
                "Stream not connected"
            ))
        }

    }

}

#[tauri::command]
pub fn gen_network_remote() {

}

#[tauri::command]
pub async fn start_transmission_server() {

    // let mut stream = TcpStream::connect("localhost:2023").await.expect("Error connecting to server");

    // let data_to_send = "Hello World from Transmitter!";
    // stream.write_all(data_to_send.as_bytes()).await.expect("Failed to write buffer to stream");

}

async fn handle_client(mut stream: TcpStream) {

    // let mut buffer = [0; 1024];

    // if let Ok(n) = stream.read(&mut buffer).await {
    //     if n == 0 {
    //         println!("Connection closed!");
    //         return; // Connection closed
    //     }

    //     let req = String::from_utf8_lossy(&buffer[..n]);
    //     println!("Received Request: {}", req);

    //     let res = "Hello, Client!".as_bytes();

    //     if let Err(e) = stream.write_all(res).await {
    //         eprintln!("Failed to write repsonse back tot he client: {}", e);
    //     }
    // } else {
    //     eprintln!("Failed to read from client!");
    // }

}

#[tauri::command]
pub async fn start_reception_server() {

    let addr = "localhost:2023";



    // let listener = TcpListener::bind(addr).await.expect("Failed to bind address");
    // println!("Listening on {}", addr);

    // while let Ok((stream, _)) = listener.accept().await {
    //     tokio::spawn(handle_client(stream));
    // }

}