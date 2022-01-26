use tokio::{io::{AsyncReadExt, AsyncWriteExt}, net::TcpListener};
  
#[tokio::main]
async fn main() {
    println!("Started");

    let listener = TcpListener::bind("localhost:8484").await.unwrap();

    let (mut socket, remote_addr) = listener.accept().await.unwrap();
    println!("remote_addr = {:?}", remote_addr);

    loop {
        let mut buffer = [0u8; 10];
        
        let read_len = socket.read(&mut buffer).await.unwrap();

        socket.write_all(&buffer[..read_len]).await.unwrap();
        println!("wrote '{}'", String::from_utf8_lossy(&buffer[..read_len]));
    }

    println!("Finished");    
}
