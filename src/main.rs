// use mini_redis::{client, Result};

// #[tokio::main]
// async fn main() -> Result<()> {
//     // Open a connection to the mini-redis address.
//     let mut client = client::connect("127.0.0.1:6379").await?;

//     // Set the key "hello" with value "world"
//     client.set("hello", "world".into()).await?;

//     // Get key "hello"
//     let result = client.get("hello").await?;

//     println!("got value from the server; result={:?}", result);

//     Ok(())
// }

use tokio::{
    io::{AsyncWriteExt, AsyncBufReadExt, BufReader},
    net::TcpListener,
};

#[tokio::main]
async fn main() {
  let listener = TcpListener::bind("localhost:8080").await.unwrap();
  loop {
    let (mut socket, _addr) = listener.accept().await.unwrap();

    let (read_stream, mut write_stream) = socket.split();
    let mut reader = BufReader::new(read_stream);
    let mut line = String::new();

    loop {
        let bytes_read = reader.read_line(&mut line).await.unwrap();
        if bytes_read == 0 {
            break;
        }
        write_stream.write_all(line.as_bytes()).await.unwrap();
        line.clear();
    }
}
}