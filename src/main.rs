use tokio::net::TcpListener;
use tokio_tungstenite::accept_async;
use tokio_tungstenite::tungstenite::protocol::Message;
use anyhow::Result;
use futures_util::{SinkExt, StreamExt};
use std::env;
use std::collections::VecDeque;

#[path ="./video/playlist.rs"]
mod playlist;
use playlist::return_playlist;

#[path ="./posterization/sorting.rs"]
mod sorting;
pub mod pixelgroup;
#[allow(unused_imports)]
use sorting::{lightness_linear, lightness_unweighted, lightness, hue, hilbert_wrapper, hilbert_lookup, fill_lookup};

mod qpixel; 

mod p_image;

static mut HILBERT_VALUES: Vec<Vec<Vec<u32>>> = Vec::new();

static mut FRAMES: VecDeque<String> = VecDeque::new();

#[tokio::main]
async fn main() -> Result<()> {
    ffmpeg_next::init().unwrap();

    let args: Vec<String> = env::args().collect();
    if args.len() == 1 {
        println!("You have not passed an url");
        std::process::exit(0);
    }
    let base_url: &str = &args[1]; //enable debug
    // println!("playing video from {}", base_url);

    let addr = "127.0.0.1:3000".to_string();
    #[allow(unused_variables)]
    let listener = TcpListener::bind(&addr).await?;
    println!("WebSocket server started on ws://{}", addr);

    fill_lookup(); // create the lookup table 
    
    // while let Ok((stream, _)) = listener.accept().await {
    //     tokio::spawn(handle_connection(stream));
    // }

    let client = reqwest::Client::new();

    let _playlist = return_playlist(client, base_url).await;

    // let _ = get_frames("./input/test.ts");

    Ok(())
}

#[allow(dead_code)]
async fn handle_connection(stream: tokio::net::TcpStream) -> Result<()> {
    let mut ws_stream = accept_async(stream).await?;
    println!("CC client connected");
    ws_stream.send(Message::Text(24.to_string())).await?;

    ws_stream.send(Message::Text("1=0xFFFFFF|0x5BCFFB|0xF5ABB9=12&159*0x80,2,2-12&159*0x80,3,3-12&159*0x80,1,1-12&159*0x80,3,3-12&159*0x80,2,2".to_string())).await?;
    ws_stream.send(Message::Text("1=0xFFFFFF|0x5BCFFB|0xF5ABB9=12&159*0x80,2,2-12&159*0x80,2,2-12&159*0x80,3,3-12&159*0x80,1,1-12&159*0x80,3,3".to_string())).await?;
    ws_stream.send(Message::Text("1=0xFFFFFF|0x5BCFFB|0xF5ABB9=12&159*0x80,3,3-12&159*0x80,2,2-12&159*0x80,2,2-12&159*0x80,3,3-12&159*0x80,1,1".to_string())).await?;
    ws_stream.send(Message::Text("1=0xFFFFFF|0x5BCFFB|0xF5ABB9=12&159*0x80,1,1-12&159*0x80,3,3-12&159*0x80,2,2-12&159*0x80,2,2-12&159*0x80,3,3".to_string())).await?;
    ws_stream.send(Message::Text("1=0xFFFFFF|0x5BCFFB|0xF5ABB9=12&159*0x80,3,3-12&159*0x80,1,1-12&159*0x80,3,3-12&159*0x80,2,2-12&159*0x80,2,2".to_string())).await?;
    
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
