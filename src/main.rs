use tokio::net::TcpListener;
use tokio_tungstenite::accept_async;
use tokio_tungstenite::tungstenite::protocol::Message;
use anyhow::Result;
use futures_util::{SinkExt, StreamExt};

#[tokio::main]
async fn main() -> Result<()> {
    let addr = "127.0.0.1:3000".to_string();
    let listener = TcpListener::bind(&addr).await?;
    println!("WebSocket server started on ws://{}", addr);

    while let Ok((stream, _)) = listener.accept().await {
        tokio::spawn(handle_connection(stream));
    }

    Ok(())
}

async fn handle_connection(stream: tokio::net::TcpStream) -> Result<()> {
    let mut ws_stream = accept_async(stream).await?;
    println!("CC client connected");
    ws_stream.send(Message::Text(24.to_string())).await?;

    ws_stream.send(Message::Text("1=0xFFFFFF|0x5BCFFB|0xF5ABB9=12&159*0x80,2,2-12&159*0x80,3,3-12&159*0x80,1,1-12&159*0x80,3,3-12&159*0x80,2,2".to_string())).await?;
    ws_stream.send(Message::Text("1=0xFFFFFF|0x5BCFFB|0xF5ABB9=12&159*0x80,2,2-12&159*0x80,2,2-12&159*0x80,3,3-12&159*0x80,1,1-12&159*0x80,3,3".to_string())).await?;
    ws_stream.send(Message::Text("1=0xFFFFFF|0x5BCFFB|0xF5ABB9=12&159*0x80,3,3-12&159*0x80,2,2-12&159*0x80,2,2-12&159*0x80,3,3-12&159*0x80,1,1".to_string())).await?;
    ws_stream.send(Message::Text("1=0xFFFFFF|0x5BCFFB|0xF5ABB9=12&159*0x80,1,1-12&159*0x80,3,3-12&159*0x80,2,2-12&159*0x80,2,2-12&159*0x80,3,3".to_string())).await?;
    ws_stream.send(Message::Text("1=0xFFFFFF|0x5BCFFB|0xF5ABB9=12&159*0x80,3,3-12&159*0x80,1,1-12&159*0x80,3,3-12&159*0x80,2,2-12&159*0x80,2,2".to_string())).await?;
    ws_stream.send(Message::Text("1=0xFFFFFF|0x5BCFFB|0xF5ABB9=12&159*0x80,2,2-12&159*0x80,3,3-12&159*0x80,1,1-12&159*0x80,3,3-12&159*0x80,2,2".to_string())).await?;
    ws_stream.send(Message::Text("1=0xFFFFFF|0x5BCFFB|0xF5ABB9=12&159*0x80,2,2-12&159*0x80,2,2-12&159*0x80,3,3-12&159*0x80,1,1-12&159*0x80,3,3".to_string())).await?;
    ws_stream.send(Message::Text("1=0xFFFFFF|0x5BCFFB|0xF5ABB9=12&159*0x80,3,3-12&159*0x80,2,2-12&159*0x80,2,2-12&159*0x80,3,3-12&159*0x80,1,1".to_string())).await?;
    ws_stream.send(Message::Text("1=0xFFFFFF|0x5BCFFB|0xF5ABB9=12&159*0x80,1,1-12&159*0x80,3,3-12&159*0x80,2,2-12&159*0x80,2,2-12&159*0x80,3,3".to_string())).await?;
    ws_stream.send(Message::Text("1=0xFFFFFF|0x5BCFFB|0xF5ABB9=12&159*0x80,3,3-12&159*0x80,1,1-12&159*0x80,3,3-12&159*0x80,2,2-12&159*0x80,2,2".to_string())).await?;    ws_stream.send(Message::Text("1=0xFFFFFF|0x5BCFFB|0xF5ABB9=12&159*0x80,2,2-12&159*0x80,3,3-12&159*0x80,1,1-12&159*0x80,3,3-12&159*0x80,2,2".to_string())).await?;
    ws_stream.send(Message::Text("1=0xFFFFFF|0x5BCFFB|0xF5ABB9=12&159*0x80,2,2-12&159*0x80,2,2-12&159*0x80,3,3-12&159*0x80,1,1-12&159*0x80,3,3".to_string())).await?;
    ws_stream.send(Message::Text("1=0xFFFFFF|0x5BCFFB|0xF5ABB9=12&159*0x80,3,3-12&159*0x80,2,2-12&159*0x80,2,2-12&159*0x80,3,3-12&159*0x80,1,1".to_string())).await?;
    ws_stream.send(Message::Text("1=0xFFFFFF|0x5BCFFB|0xF5ABB9=12&159*0x80,1,1-12&159*0x80,3,3-12&159*0x80,2,2-12&159*0x80,2,2-12&159*0x80,3,3".to_string())).await?;
    ws_stream.send(Message::Text("1=0xFFFFFF|0x5BCFFB|0xF5ABB9=12&159*0x80,3,3-12&159*0x80,1,1-12&159*0x80,3,3-12&159*0x80,2,2-12&159*0x80,2,2".to_string())).await?;    ws_stream.send(Message::Text("1=0xFFFFFF|0x5BCFFB|0xF5ABB9=12&159*0x80,2,2-12&159*0x80,3,3-12&159*0x80,1,1-12&159*0x80,3,3-12&159*0x80,2,2".to_string())).await?;
    ws_stream.send(Message::Text("1=0xFFFFFF|0x5BCFFB|0xF5ABB9=12&159*0x80,2,2-12&159*0x80,2,2-12&159*0x80,3,3-12&159*0x80,1,1-12&159*0x80,3,3".to_string())).await?;
    ws_stream.send(Message::Text("1=0xFFFFFF|0x5BCFFB|0xF5ABB9=12&159*0x80,3,3-12&159*0x80,2,2-12&159*0x80,2,2-12&159*0x80,3,3-12&159*0x80,1,1".to_string())).await?;
    ws_stream.send(Message::Text("1=0xFFFFFF|0x5BCFFB|0xF5ABB9=12&159*0x80,1,1-12&159*0x80,3,3-12&159*0x80,2,2-12&159*0x80,2,2-12&159*0x80,3,3".to_string())).await?;
    ws_stream.send(Message::Text("1=0xFFFFFF|0x5BCFFB|0xF5ABB9=12&159*0x80,3,3-12&159*0x80,1,1-12&159*0x80,3,3-12&159*0x80,2,2-12&159*0x80,2,2".to_string())).await?;    ws_stream.send(Message::Text("1=0xFFFFFF|0x5BCFFB|0xF5ABB9=12&159*0x80,2,2-12&159*0x80,3,3-12&159*0x80,1,1-12&159*0x80,3,3-12&159*0x80,2,2".to_string())).await?;
    ws_stream.send(Message::Text("1=0xFFFFFF|0x5BCFFB|0xF5ABB9=12&159*0x80,2,2-12&159*0x80,2,2-12&159*0x80,3,3-12&159*0x80,1,1-12&159*0x80,3,3".to_string())).await?;
    ws_stream.send(Message::Text("1=0xFFFFFF|0x5BCFFB|0xF5ABB9=12&159*0x80,3,3-12&159*0x80,2,2-12&159*0x80,2,2-12&159*0x80,3,3-12&159*0x80,1,1".to_string())).await?;
    ws_stream.send(Message::Text("1=0xFFFFFF|0x5BCFFB|0xF5ABB9=12&159*0x80,1,1-12&159*0x80,3,3-12&159*0x80,2,2-12&159*0x80,2,2-12&159*0x80,3,3".to_string())).await?;
    ws_stream.send(Message::Text("1=0xFFFFFF|0x5BCFFB|0xF5ABB9=12&159*0x80,3,3-12&159*0x80,1,1-12&159*0x80,3,3-12&159*0x80,2,2-12&159*0x80,2,2".to_string())).await?;    ws_stream.send(Message::Text("1=0xFFFFFF|0x5BCFFB|0xF5ABB9=12&159*0x80,2,2-12&159*0x80,3,3-12&159*0x80,1,1-12&159*0x80,3,3-12&159*0x80,2,2".to_string())).await?;
    ws_stream.send(Message::Text("1=0xFFFFFF|0x5BCFFB|0xF5ABB9=12&159*0x80,2,2-12&159*0x80,2,2-12&159*0x80,3,3-12&159*0x80,1,1-12&159*0x80,3,3".to_string())).await?;
    ws_stream.send(Message::Text("1=0xFFFFFF|0x5BCFFB|0xF5ABB9=12&159*0x80,3,3-12&159*0x80,2,2-12&159*0x80,2,2-12&159*0x80,3,3-12&159*0x80,1,1".to_string())).await?;
    ws_stream.send(Message::Text("1=0xFFFFFF|0x5BCFFB|0xF5ABB9=12&159*0x80,1,1-12&159*0x80,3,3-12&159*0x80,2,2-12&159*0x80,2,2-12&159*0x80,3,3".to_string())).await?;
    ws_stream.send(Message::Text("1=0xFFFFFF|0x5BCFFB|0xF5ABB9=12&159*0x80,3,3-12&159*0x80,1,1-12&159*0x80,3,3-12&159*0x80,2,2-12&159*0x80,2,2".to_string())).await?;    ws_stream.send(Message::Text("1=0xFFFFFF|0x5BCFFB|0xF5ABB9=12&159*0x80,2,2-12&159*0x80,3,3-12&159*0x80,1,1-12&159*0x80,3,3-12&159*0x80,2,2".to_string())).await?;
    ws_stream.send(Message::Text("1=0xFFFFFF|0x5BCFFB|0xF5ABB9=12&159*0x80,2,2-12&159*0x80,2,2-12&159*0x80,3,3-12&159*0x80,1,1-12&159*0x80,3,3".to_string())).await?;
    ws_stream.send(Message::Text("1=0xFFFFFF|0x5BCFFB|0xF5ABB9=12&159*0x80,3,3-12&159*0x80,2,2-12&159*0x80,2,2-12&159*0x80,3,3-12&159*0x80,1,1".to_string())).await?;
    ws_stream.send(Message::Text("1=0xFFFFFF|0x5BCFFB|0xF5ABB9=12&159*0x80,1,1-12&159*0x80,3,3-12&159*0x80,2,2-12&159*0x80,2,2-12&159*0x80,3,3".to_string())).await?;
    ws_stream.send(Message::Text("1=0xFFFFFF|0x5BCFFB|0xF5ABB9=12&159*0x80,3,3-12&159*0x80,1,1-12&159*0x80,3,3-12&159*0x80,2,2-12&159*0x80,2,2".to_string())).await?;    ws_stream.send(Message::Text("1=0xFFFFFF|0x5BCFFB|0xF5ABB9=12&159*0x80,2,2-12&159*0x80,3,3-12&159*0x80,1,1-12&159*0x80,3,3-12&159*0x80,2,2".to_string())).await?;
    ws_stream.send(Message::Text("1=0xFFFFFF|0x5BCFFB|0xF5ABB9=12&159*0x80,2,2-12&159*0x80,2,2-12&159*0x80,3,3-12&159*0x80,1,1-12&159*0x80,3,3".to_string())).await?;
    ws_stream.send(Message::Text("1=0xFFFFFF|0x5BCFFB|0xF5ABB9=12&159*0x80,3,3-12&159*0x80,2,2-12&159*0x80,2,2-12&159*0x80,3,3-12&159*0x80,1,1".to_string())).await?;
    ws_stream.send(Message::Text("1=0xFFFFFF|0x5BCFFB|0xF5ABB9=12&159*0x80,1,1-12&159*0x80,3,3-12&159*0x80,2,2-12&159*0x80,2,2-12&159*0x80,3,3".to_string())).await?;
    ws_stream.send(Message::Text("1=0xFFFFFF|0x5BCFFB|0xF5ABB9=12&159*0x80,3,3-12&159*0x80,1,1-12&159*0x80,3,3-12&159*0x80,2,2-12&159*0x80,2,2".to_string())).await?;    ws_stream.send(Message::Text("1=0xFFFFFF|0x5BCFFB|0xF5ABB9=12&159*0x80,2,2-12&159*0x80,3,3-12&159*0x80,1,1-12&159*0x80,3,3-12&159*0x80,2,2".to_string())).await?;
    ws_stream.send(Message::Text("1=0xFFFFFF|0x5BCFFB|0xF5ABB9=12&159*0x80,2,2-12&159*0x80,2,2-12&159*0x80,3,3-12&159*0x80,1,1-12&159*0x80,3,3".to_string())).await?;
    ws_stream.send(Message::Text("1=0xFFFFFF|0x5BCFFB|0xF5ABB9=12&159*0x80,3,3-12&159*0x80,2,2-12&159*0x80,2,2-12&159*0x80,3,3-12&159*0x80,1,1".to_string())).await?;
    ws_stream.send(Message::Text("1=0xFFFFFF|0x5BCFFB|0xF5ABB9=12&159*0x80,1,1-12&159*0x80,3,3-12&159*0x80,2,2-12&159*0x80,2,2-12&159*0x80,3,3".to_string())).await?;
    ws_stream.send(Message::Text("1=0xFFFFFF|0x5BCFFB|0xF5ABB9=12&159*0x80,3,3-12&159*0x80,1,1-12&159*0x80,3,3-12&159*0x80,2,2-12&159*0x80,2,2".to_string())).await?;    ws_stream.send(Message::Text("1=0xFFFFFF|0x5BCFFB|0xF5ABB9=12&159*0x80,2,2-12&159*0x80,3,3-12&159*0x80,1,1-12&159*0x80,3,3-12&159*0x80,2,2".to_string())).await?;
    ws_stream.send(Message::Text("1=0xFFFFFF|0x5BCFFB|0xF5ABB9=12&159*0x80,2,2-12&159*0x80,2,2-12&159*0x80,3,3-12&159*0x80,1,1-12&159*0x80,3,3".to_string())).await?;
    ws_stream.send(Message::Text("1=0xFFFFFF|0x5BCFFB|0xF5ABB9=12&159*0x80,3,3-12&159*0x80,2,2-12&159*0x80,2,2-12&159*0x80,3,3-12&159*0x80,1,1".to_string())).await?;
    ws_stream.send(Message::Text("1=0xFFFFFF|0x5BCFFB|0xF5ABB9=12&159*0x80,1,1-12&159*0x80,3,3-12&159*0x80,2,2-12&159*0x80,2,2-12&159*0x80,3,3".to_string())).await?;
    ws_stream.send(Message::Text("1=0xFFFFFF|0x5BCFFB|0xF5ABB9=12&159*0x80,3,3-12&159*0x80,1,1-12&159*0x80,3,3-12&159*0x80,2,2-12&159*0x80,2,2".to_string())).await?;    ws_stream.send(Message::Text("1=0xFFFFFF|0x5BCFFB|0xF5ABB9=12&159*0x80,2,2-12&159*0x80,3,3-12&159*0x80,1,1-12&159*0x80,3,3-12&159*0x80,2,2".to_string())).await?;
    ws_stream.send(Message::Text("1=0xFFFFFF|0x5BCFFB|0xF5ABB9=12&159*0x80,2,2-12&159*0x80,2,2-12&159*0x80,3,3-12&159*0x80,1,1-12&159*0x80,3,3".to_string())).await?;
    ws_stream.send(Message::Text("1=0xFFFFFF|0x5BCFFB|0xF5ABB9=12&159*0x80,3,3-12&159*0x80,2,2-12&159*0x80,2,2-12&159*0x80,3,3-12&159*0x80,1,1".to_string())).await?;
    ws_stream.send(Message::Text("1=0xFFFFFF|0x5BCFFB|0xF5ABB9=12&159*0x80,1,1-12&159*0x80,3,3-12&159*0x80,2,2-12&159*0x80,2,2-12&159*0x80,3,3".to_string())).await?;
    ws_stream.send(Message::Text("1=0xFFFFFF|0x5BCFFB|0xF5ABB9=12&159*0x80,3,3-12&159*0x80,1,1-12&159*0x80,3,3-12&159*0x80,2,2-12&159*0x80,2,2".to_string())).await?;    ws_stream.send(Message::Text("1=0xFFFFFF|0x5BCFFB|0xF5ABB9=12&159*0x80,2,2-12&159*0x80,3,3-12&159*0x80,1,1-12&159*0x80,3,3-12&159*0x80,2,2".to_string())).await?;
    ws_stream.send(Message::Text("1=0xFFFFFF|0x5BCFFB|0xF5ABB9=12&159*0x80,2,2-12&159*0x80,2,2-12&159*0x80,3,3-12&159*0x80,1,1-12&159*0x80,3,3".to_string())).await?;
    ws_stream.send(Message::Text("1=0xFFFFFF|0x5BCFFB|0xF5ABB9=12&159*0x80,3,3-12&159*0x80,2,2-12&159*0x80,2,2-12&159*0x80,3,3-12&159*0x80,1,1".to_string())).await?;
    ws_stream.send(Message::Text("1=0xFFFFFF|0x5BCFFB|0xF5ABB9=12&159*0x80,1,1-12&159*0x80,3,3-12&159*0x80,2,2-12&159*0x80,2,2-12&159*0x80,3,3".to_string())).await?;
    ws_stream.send(Message::Text("1=0xFFFFFF|0x5BCFFB|0xF5ABB9=12&159*0x80,3,3-12&159*0x80,1,1-12&159*0x80,3,3-12&159*0x80,2,2-12&159*0x80,2,2".to_string())).await?;    ws_stream.send(Message::Text("1=0xFFFFFF|0x5BCFFB|0xF5ABB9=12&159*0x80,2,2-12&159*0x80,3,3-12&159*0x80,1,1-12&159*0x80,3,3-12&159*0x80,2,2".to_string())).await?;
    ws_stream.send(Message::Text("1=0xFFFFFF|0x5BCFFB|0xF5ABB9=12&159*0x80,2,2-12&159*0x80,2,2-12&159*0x80,3,3-12&159*0x80,1,1-12&159*0x80,3,3".to_string())).await?;
    ws_stream.send(Message::Text("1=0xFFFFFF|0x5BCFFB|0xF5ABB9=12&159*0x80,3,3-12&159*0x80,2,2-12&159*0x80,2,2-12&159*0x80,3,3-12&159*0x80,1,1".to_string())).await?;
    ws_stream.send(Message::Text("1=0xFFFFFF|0x5BCFFB|0xF5ABB9=12&159*0x80,1,1-12&159*0x80,3,3-12&159*0x80,2,2-12&159*0x80,2,2-12&159*0x80,3,3".to_string())).await?;
    ws_stream.send(Message::Text("1=0xFFFFFF|0x5BCFFB|0xF5ABB9=12&159*0x80,3,3-12&159*0x80,1,1-12&159*0x80,3,3-12&159*0x80,2,2-12&159*0x80,2,2".to_string())).await?;    ws_stream.send(Message::Text("1=0xFFFFFF|0x5BCFFB|0xF5ABB9=12&159*0x80,2,2-12&159*0x80,3,3-12&159*0x80,1,1-12&159*0x80,3,3-12&159*0x80,2,2".to_string())).await?;
    ws_stream.send(Message::Text("1=0xFFFFFF|0x5BCFFB|0xF5ABB9=12&159*0x80,2,2-12&159*0x80,2,2-12&159*0x80,3,3-12&159*0x80,1,1-12&159*0x80,3,3".to_string())).await?;
    ws_stream.send(Message::Text("1=0xFFFFFF|0x5BCFFB|0xF5ABB9=12&159*0x80,3,3-12&159*0x80,2,2-12&159*0x80,2,2-12&159*0x80,3,3-12&159*0x80,1,1".to_string())).await?;
    ws_stream.send(Message::Text("1=0xFFFFFF|0x5BCFFB|0xF5ABB9=12&159*0x80,1,1-12&159*0x80,3,3-12&159*0x80,2,2-12&159*0x80,2,2-12&159*0x80,3,3".to_string())).await?;
    ws_stream.send(Message::Text("1=0xFFFFFF|0x5BCFFB|0xF5ABB9=12&159*0x80,3,3-12&159*0x80,1,1-12&159*0x80,3,3-12&159*0x80,2,2-12&159*0x80,2,2".to_string())).await?;    ws_stream.send(Message::Text("1=0xFFFFFF|0x5BCFFB|0xF5ABB9=12&159*0x80,2,2-12&159*0x80,3,3-12&159*0x80,1,1-12&159*0x80,3,3-12&159*0x80,2,2".to_string())).await?;
    ws_stream.send(Message::Text("1=0xFFFFFF|0x5BCFFB|0xF5ABB9=12&159*0x80,2,2-12&159*0x80,2,2-12&159*0x80,3,3-12&159*0x80,1,1-12&159*0x80,3,3".to_string())).await?;
    ws_stream.send(Message::Text("1=0xFFFFFF|0x5BCFFB|0xF5ABB9=12&159*0x80,3,3-12&159*0x80,2,2-12&159*0x80,2,2-12&159*0x80,3,3-12&159*0x80,1,1".to_string())).await?;
    ws_stream.send(Message::Text("1=0xFFFFFF|0x5BCFFB|0xF5ABB9=12&159*0x80,1,1-12&159*0x80,3,3-12&159*0x80,2,2-12&159*0x80,2,2-12&159*0x80,3,3".to_string())).await?;
    ws_stream.send(Message::Text("1=0xFFFFFF|0x5BCFFB|0xF5ABB9=12&159*0x80,3,3-12&159*0x80,1,1-12&159*0x80,3,3-12&159*0x80,2,2-12&159*0x80,2,2".to_string())).await?;    ws_stream.send(Message::Text("1=0xFFFFFF|0x5BCFFB|0xF5ABB9=12&159*0x80,2,2-12&159*0x80,3,3-12&159*0x80,1,1-12&159*0x80,3,3-12&159*0x80,2,2".to_string())).await?;
    ws_stream.send(Message::Text("1=0xFFFFFF|0x5BCFFB|0xF5ABB9=12&159*0x80,2,2-12&159*0x80,2,2-12&159*0x80,3,3-12&159*0x80,1,1-12&159*0x80,3,3".to_string())).await?;
    ws_stream.send(Message::Text("1=0xFFFFFF|0x5BCFFB|0xF5ABB9=12&159*0x80,3,3-12&159*0x80,2,2-12&159*0x80,2,2-12&159*0x80,3,3-12&159*0x80,1,1".to_string())).await?;
    ws_stream.send(Message::Text("1=0xFFFFFF|0x5BCFFB|0xF5ABB9=12&159*0x80,1,1-12&159*0x80,3,3-12&159*0x80,2,2-12&159*0x80,2,2-12&159*0x80,3,3".to_string())).await?;
    ws_stream.send(Message::Text("1=0xFFFFFF|0x5BCFFB|0xF5ABB9=12&159*0x80,3,3-12&159*0x80,1,1-12&159*0x80,3,3-12&159*0x80,2,2-12&159*0x80,2,2".to_string())).await?;    ws_stream.send(Message::Text("1=0xFFFFFF|0x5BCFFB|0xF5ABB9=12&159*0x80,2,2-12&159*0x80,3,3-12&159*0x80,1,1-12&159*0x80,3,3-12&159*0x80,2,2".to_string())).await?;
    ws_stream.send(Message::Text("1=0xFFFFFF|0x5BCFFB|0xF5ABB9=12&159*0x80,2,2-12&159*0x80,2,2-12&159*0x80,3,3-12&159*0x80,1,1-12&159*0x80,3,3".to_string())).await?;
    ws_stream.send(Message::Text("1=0xFFFFFF|0x5BCFFB|0xF5ABB9=12&159*0x80,3,3-12&159*0x80,2,2-12&159*0x80,2,2-12&159*0x80,3,3-12&159*0x80,1,1".to_string())).await?;
    ws_stream.send(Message::Text("1=0xFFFFFF|0x5BCFFB|0xF5ABB9=12&159*0x80,1,1-12&159*0x80,3,3-12&159*0x80,2,2-12&159*0x80,2,2-12&159*0x80,3,3".to_string())).await?;
    ws_stream.send(Message::Text("1=0xFFFFFF|0x5BCFFB|0xF5ABB9=12&159*0x80,3,3-12&159*0x80,1,1-12&159*0x80,3,3-12&159*0x80,2,2-12&159*0x80,2,2".to_string())).await?;    ws_stream.send(Message::Text("1=0xFFFFFF|0x5BCFFB|0xF5ABB9=12&159*0x80,2,2-12&159*0x80,3,3-12&159*0x80,1,1-12&159*0x80,3,3-12&159*0x80,2,2".to_string())).await?;
    ws_stream.send(Message::Text("1=0xFFFFFF|0x5BCFFB|0xF5ABB9=12&159*0x80,2,2-12&159*0x80,2,2-12&159*0x80,3,3-12&159*0x80,1,1-12&159*0x80,3,3".to_string())).await?;
    ws_stream.send(Message::Text("1=0xFFFFFF|0x5BCFFB|0xF5ABB9=12&159*0x80,3,3-12&159*0x80,2,2-12&159*0x80,2,2-12&159*0x80,3,3-12&159*0x80,1,1".to_string())).await?;
    ws_stream.send(Message::Text("1=0xFFFFFF|0x5BCFFB|0xF5ABB9=12&159*0x80,1,1-12&159*0x80,3,3-12&159*0x80,2,2-12&159*0x80,2,2-12&159*0x80,3,3".to_string())).await?;
    ws_stream.send(Message::Text("1=0xFFFFFF|0x5BCFFB|0xF5ABB9=12&159*0x80,3,3-12&159*0x80,1,1-12&159*0x80,3,3-12&159*0x80,2,2-12&159*0x80,2,2".to_string())).await?;    ws_stream.send(Message::Text("1=0xFFFFFF|0x5BCFFB|0xF5ABB9=12&159*0x80,2,2-12&159*0x80,3,3-12&159*0x80,1,1-12&159*0x80,3,3-12&159*0x80,2,2".to_string())).await?;
    ws_stream.send(Message::Text("1=0xFFFFFF|0x5BCFFB|0xF5ABB9=12&159*0x80,2,2-12&159*0x80,2,2-12&159*0x80,3,3-12&159*0x80,1,1-12&159*0x80,3,3".to_string())).await?;
    ws_stream.send(Message::Text("1=0xFFFFFF|0x5BCFFB|0xF5ABB9=12&159*0x80,3,3-12&159*0x80,2,2-12&159*0x80,2,2-12&159*0x80,3,3-12&159*0x80,1,1".to_string())).await?;
    ws_stream.send(Message::Text("1=0xFFFFFF|0x5BCFFB|0xF5ABB9=12&159*0x80,1,1-12&159*0x80,3,3-12&159*0x80,2,2-12&159*0x80,2,2-12&159*0x80,3,3".to_string())).await?;
    ws_stream.send(Message::Text("1=0xFFFFFF|0x5BCFFB|0xF5ABB9=12&159*0x80,3,3-12&159*0x80,1,1-12&159*0x80,3,3-12&159*0x80,2,2-12&159*0x80,2,2".to_string())).await?;    ws_stream.send(Message::Text("1=0xFFFFFF|0x5BCFFB|0xF5ABB9=12&159*0x80,2,2-12&159*0x80,3,3-12&159*0x80,1,1-12&159*0x80,3,3-12&159*0x80,2,2".to_string())).await?;
    ws_stream.send(Message::Text("1=0xFFFFFF|0x5BCFFB|0xF5ABB9=12&159*0x80,2,2-12&159*0x80,2,2-12&159*0x80,3,3-12&159*0x80,1,1-12&159*0x80,3,3".to_string())).await?;
    ws_stream.send(Message::Text("1=0xFFFFFF|0x5BCFFB|0xF5ABB9=12&159*0x80,3,3-12&159*0x80,2,2-12&159*0x80,2,2-12&159*0x80,3,3-12&159*0x80,1,1".to_string())).await?;
    ws_stream.send(Message::Text("1=0xFFFFFF|0x5BCFFB|0xF5ABB9=12&159*0x80,1,1-12&159*0x80,3,3-12&159*0x80,2,2-12&159*0x80,2,2-12&159*0x80,3,3".to_string())).await?;
    ws_stream.send(Message::Text("1=0xFFFFFF|0x5BCFFB|0xF5ABB9=12&159*0x80,3,3-12&159*0x80,1,1-12&159*0x80,3,3-12&159*0x80,2,2-12&159*0x80,2,2".to_string())).await?;    ws_stream.send(Message::Text("1=0xFFFFFF|0x5BCFFB|0xF5ABB9=12&159*0x80,2,2-12&159*0x80,3,3-12&159*0x80,1,1-12&159*0x80,3,3-12&159*0x80,2,2".to_string())).await?;
    ws_stream.send(Message::Text("1=0xFFFFFF|0x5BCFFB|0xF5ABB9=12&159*0x80,2,2-12&159*0x80,2,2-12&159*0x80,3,3-12&159*0x80,1,1-12&159*0x80,3,3".to_string())).await?;
    ws_stream.send(Message::Text("1=0xFFFFFF|0x5BCFFB|0xF5ABB9=12&159*0x80,3,3-12&159*0x80,2,2-12&159*0x80,2,2-12&159*0x80,3,3-12&159*0x80,1,1".to_string())).await?;
    ws_stream.send(Message::Text("1=0xFFFFFF|0x5BCFFB|0xF5ABB9=12&159*0x80,1,1-12&159*0x80,3,3-12&159*0x80,2,2-12&159*0x80,2,2-12&159*0x80,3,3".to_string())).await?;
    ws_stream.send(Message::Text("1=0xFFFFFF|0x5BCFFB|0xF5ABB9=12&159*0x80,3,3-12&159*0x80,1,1-12&159*0x80,3,3-12&159*0x80,2,2-12&159*0x80,2,2".to_string())).await?;    ws_stream.send(Message::Text("1=0xFFFFFF|0x5BCFFB|0xF5ABB9=12&159*0x80,2,2-12&159*0x80,3,3-12&159*0x80,1,1-12&159*0x80,3,3-12&159*0x80,2,2".to_string())).await?;
    ws_stream.send(Message::Text("1=0xFFFFFF|0x5BCFFB|0xF5ABB9=12&159*0x80,2,2-12&159*0x80,2,2-12&159*0x80,3,3-12&159*0x80,1,1-12&159*0x80,3,3".to_string())).await?;
    ws_stream.send(Message::Text("1=0xFFFFFF|0x5BCFFB|0xF5ABB9=12&159*0x80,3,3-12&159*0x80,2,2-12&159*0x80,2,2-12&159*0x80,3,3-12&159*0x80,1,1".to_string())).await?;
    ws_stream.send(Message::Text("1=0xFFFFFF|0x5BCFFB|0xF5ABB9=12&159*0x80,1,1-12&159*0x80,3,3-12&159*0x80,2,2-12&159*0x80,2,2-12&159*0x80,3,3".to_string())).await?;
    ws_stream.send(Message::Text("1=0xFFFFFF|0x5BCFFB|0xF5ABB9=12&159*0x80,3,3-12&159*0x80,1,1-12&159*0x80,3,3-12&159*0x80,2,2-12&159*0x80,2,2".to_string())).await?;    ws_stream.send(Message::Text("1=0xFFFFFF|0x5BCFFB|0xF5ABB9=12&159*0x80,2,2-12&159*0x80,3,3-12&159*0x80,1,1-12&159*0x80,3,3-12&159*0x80,2,2".to_string())).await?;
    ws_stream.send(Message::Text("1=0xFFFFFF|0x5BCFFB|0xF5ABB9=12&159*0x80,2,2-12&159*0x80,2,2-12&159*0x80,3,3-12&159*0x80,1,1-12&159*0x80,3,3".to_string())).await?;
    ws_stream.send(Message::Text("1=0xFFFFFF|0x5BCFFB|0xF5ABB9=12&159*0x80,3,3-12&159*0x80,2,2-12&159*0x80,2,2-12&159*0x80,3,3-12&159*0x80,1,1".to_string())).await?;
    ws_stream.send(Message::Text("1=0xFFFFFF|0x5BCFFB|0xF5ABB9=12&159*0x80,1,1-12&159*0x80,3,3-12&159*0x80,2,2-12&159*0x80,2,2-12&159*0x80,3,3".to_string())).await?;
    ws_stream.send(Message::Text("1=0xFFFFFF|0x5BCFFB|0xF5ABB9=12&159*0x80,3,3-12&159*0x80,1,1-12&159*0x80,3,3-12&159*0x80,2,2-12&159*0x80,2,2".to_string())).await?;    ws_stream.send(Message::Text("1=0xFFFFFF|0x5BCFFB|0xF5ABB9=12&159*0x80,2,2-12&159*0x80,3,3-12&159*0x80,1,1-12&159*0x80,3,3-12&159*0x80,2,2".to_string())).await?;
    ws_stream.send(Message::Text("1=0xFFFFFF|0x5BCFFB|0xF5ABB9=12&159*0x80,2,2-12&159*0x80,2,2-12&159*0x80,3,3-12&159*0x80,1,1-12&159*0x80,3,3".to_string())).await?;
    ws_stream.send(Message::Text("1=0xFFFFFF|0x5BCFFB|0xF5ABB9=12&159*0x80,3,3-12&159*0x80,2,2-12&159*0x80,2,2-12&159*0x80,3,3-12&159*0x80,1,1".to_string())).await?;
    ws_stream.send(Message::Text("1=0xFFFFFF|0x5BCFFB|0xF5ABB9=12&159*0x80,1,1-12&159*0x80,3,3-12&159*0x80,2,2-12&159*0x80,2,2-12&159*0x80,3,3".to_string())).await?;
    ws_stream.send(Message::Text("1=0xFFFFFF|0x5BCFFB|0xF5ABB9=12&159*0x80,3,3-12&159*0x80,1,1-12&159*0x80,3,3-12&159*0x80,2,2-12&159*0x80,2,2".to_string())).await?;    ws_stream.send(Message::Text("1=0xFFFFFF|0x5BCFFB|0xF5ABB9=12&159*0x80,2,2-12&159*0x80,3,3-12&159*0x80,1,1-12&159*0x80,3,3-12&159*0x80,2,2".to_string())).await?;
    ws_stream.send(Message::Text("1=0xFFFFFF|0x5BCFFB|0xF5ABB9=12&159*0x80,2,2-12&159*0x80,2,2-12&159*0x80,3,3-12&159*0x80,1,1-12&159*0x80,3,3".to_string())).await?;
    ws_stream.send(Message::Text("1=0xFFFFFF|0x5BCFFB|0xF5ABB9=12&159*0x80,3,3-12&159*0x80,2,2-12&159*0x80,2,2-12&159*0x80,3,3-12&159*0x80,1,1".to_string())).await?;
    ws_stream.send(Message::Text("1=0xFFFFFF|0x5BCFFB|0xF5ABB9=12&159*0x80,1,1-12&159*0x80,3,3-12&159*0x80,2,2-12&159*0x80,2,2-12&159*0x80,3,3".to_string())).await?;
    ws_stream.send(Message::Text("1=0xFFFFFF|0x5BCFFB|0xF5ABB9=12&159*0x80,3,3-12&159*0x80,1,1-12&159*0x80,3,3-12&159*0x80,2,2-12&159*0x80,2,2".to_string())).await?;    ws_stream.send(Message::Text("1=0xFFFFFF|0x5BCFFB|0xF5ABB9=12&159*0x80,2,2-12&159*0x80,3,3-12&159*0x80,1,1-12&159*0x80,3,3-12&159*0x80,2,2".to_string())).await?;
    ws_stream.send(Message::Text("1=0xFFFFFF|0x5BCFFB|0xF5ABB9=12&159*0x80,2,2-12&159*0x80,2,2-12&159*0x80,3,3-12&159*0x80,1,1-12&159*0x80,3,3".to_string())).await?;
    ws_stream.send(Message::Text("1=0xFFFFFF|0x5BCFFB|0xF5ABB9=12&159*0x80,3,3-12&159*0x80,2,2-12&159*0x80,2,2-12&159*0x80,3,3-12&159*0x80,1,1".to_string())).await?;
    ws_stream.send(Message::Text("1=0xFFFFFF|0x5BCFFB|0xF5ABB9=12&159*0x80,1,1-12&159*0x80,3,3-12&159*0x80,2,2-12&159*0x80,2,2-12&159*0x80,3,3".to_string())).await?;
    ws_stream.send(Message::Text("1=0xFFFFFF|0x5BCFFB|0xF5ABB9=12&159*0x80,3,3-12&159*0x80,1,1-12&159*0x80,3,3-12&159*0x80,2,2-12&159*0x80,2,2".to_string())).await?;    ws_stream.send(Message::Text("1=0xFFFFFF|0x5BCFFB|0xF5ABB9=12&159*0x80,2,2-12&159*0x80,3,3-12&159*0x80,1,1-12&159*0x80,3,3-12&159*0x80,2,2".to_string())).await?;
    ws_stream.send(Message::Text("1=0xFFFFFF|0x5BCFFB|0xF5ABB9=12&159*0x80,2,2-12&159*0x80,2,2-12&159*0x80,3,3-12&159*0x80,1,1-12&159*0x80,3,3".to_string())).await?;
    ws_stream.send(Message::Text("1=0xFFFFFF|0x5BCFFB|0xF5ABB9=12&159*0x80,3,3-12&159*0x80,2,2-12&159*0x80,2,2-12&159*0x80,3,3-12&159*0x80,1,1".to_string())).await?;
    ws_stream.send(Message::Text("1=0xFFFFFF|0x5BCFFB|0xF5ABB9=12&159*0x80,1,1-12&159*0x80,3,3-12&159*0x80,2,2-12&159*0x80,2,2-12&159*0x80,3,3".to_string())).await?;
    ws_stream.send(Message::Text("1=0xFFFFFF|0x5BCFFB|0xF5ABB9=12&159*0x80,3,3-12&159*0x80,1,1-12&159*0x80,3,3-12&159*0x80,2,2-12&159*0x80,2,2".to_string())).await?;    ws_stream.send(Message::Text("1=0xFFFFFF|0x5BCFFB|0xF5ABB9=12&159*0x80,2,2-12&159*0x80,3,3-12&159*0x80,1,1-12&159*0x80,3,3-12&159*0x80,2,2".to_string())).await?;
    ws_stream.send(Message::Text("1=0xFFFFFF|0x5BCFFB|0xF5ABB9=12&159*0x80,2,2-12&159*0x80,2,2-12&159*0x80,3,3-12&159*0x80,1,1-12&159*0x80,3,3".to_string())).await?;
    ws_stream.send(Message::Text("1=0xFFFFFF|0x5BCFFB|0xF5ABB9=12&159*0x80,3,3-12&159*0x80,2,2-12&159*0x80,2,2-12&159*0x80,3,3-12&159*0x80,1,1".to_string())).await?;
    ws_stream.send(Message::Text("1=0xFFFFFF|0x5BCFFB|0xF5ABB9=12&159*0x80,1,1-12&159*0x80,3,3-12&159*0x80,2,2-12&159*0x80,2,2-12&159*0x80,3,3".to_string())).await?;
    ws_stream.send(Message::Text("1=0xFFFFFF|0x5BCFFB|0xF5ABB9=12&159*0x80,3,3-12&159*0x80,1,1-12&159*0x80,3,3-12&159*0x80,2,2-12&159*0x80,2,2".to_string())).await?;    ws_stream.send(Message::Text("1=0xFFFFFF|0x5BCFFB|0xF5ABB9=12&159*0x80,2,2-12&159*0x80,3,3-12&159*0x80,1,1-12&159*0x80,3,3-12&159*0x80,2,2".to_string())).await?;
    ws_stream.send(Message::Text("1=0xFFFFFF|0x5BCFFB|0xF5ABB9=12&159*0x80,2,2-12&159*0x80,2,2-12&159*0x80,3,3-12&159*0x80,1,1-12&159*0x80,3,3".to_string())).await?;
    ws_stream.send(Message::Text("1=0xFFFFFF|0x5BCFFB|0xF5ABB9=12&159*0x80,3,3-12&159*0x80,2,2-12&159*0x80,2,2-12&159*0x80,3,3-12&159*0x80,1,1".to_string())).await?;
    ws_stream.send(Message::Text("1=0xFFFFFF|0x5BCFFB|0xF5ABB9=12&159*0x80,1,1-12&159*0x80,3,3-12&159*0x80,2,2-12&159*0x80,2,2-12&159*0x80,3,3".to_string())).await?;
    ws_stream.send(Message::Text("1=0xFFFFFF|0x5BCFFB|0xF5ABB9=12&159*0x80,3,3-12&159*0x80,1,1-12&159*0x80,3,3-12&159*0x80,2,2-12&159*0x80,2,2".to_string())).await?;
    ws_stream.send(Message::Text("1=0xFFFFFF|0x5BCFFB|0xF5ABB9=12&159*0x80,2,2-12&159*0x80,3,3-12&159*0x80,1,1-12&159*0x80,3,3-12&159*0x80,2,2".to_string())).await?;
    

    ws_stream.send(Message::Text("eof".to_string())).await?;

    while let Some(msg) = ws_stream.next().await {
        let msg = msg?;
        if msg.is_text() {
            let received_text = msg.to_text()?;
            println!("Received message: {}", received_text);
            ws_stream.send(Message::Text(received_text.to_string())).await?;
        }
    }

    Ok(())
}
