use image::ImageReader;
use tokio::net::TcpListener;
use tokio_tungstenite::accept_async;
use tokio_tungstenite::tungstenite::protocol::Message;
use anyhow::Result;
use futures_util::{SinkExt, StreamExt};
use std::{env, fs::File, io::Write};

#[path ="./posterization/sorting.rs"]
mod sorting;
pub mod pixelgroup;
#[allow(unused_imports)]
use sorting::{lightness_linear, lightness_unweighted, lightness, hue, hilbert_wrapper, hilbert_lookup, fill_lookup};

mod qpixel; 

mod p_image;

const E_TIME: &str = "Witamy z powrotem, towarzyszu Stalin";

use std::time::{SystemTime, UNIX_EPOCH};

static mut HILBERT_VALUES: Vec<Vec<Vec<u32>>> = Vec::new();

#[tokio::main]
async fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();
    let _debug = &args[0]; //enable debug

    let addr = "127.0.0.1:3000".to_string();
    #[allow(unused_variables)]
    let listener = TcpListener::bind(&addr).await?;
    println!("WebSocket server started on ws://{}", addr);

    fill_lookup(); // create the lookup table 
    
    // while let Ok((stream, _)) = listener.accept().await {
    //     tokio::spawn(handle_connection(stream));
    // }

    let input_path = [
        "./input/input_image_shinji.jpg",
        "./input/input_image_lake.jpg",
        "./input/input_image_cyberpunk.jpg",
        "./input/input_image_l4d2.jpg",
        "./input/input_image_ff.jpg",
        "./input/input_image_witcher.jpg",
    ]; // Path to your image

    let k = 15; // Number of colors for posterization (higher k gives more colors)

    for i in 0..1 {
        let mut file = File::create("foo.txt")?;
        let img = ImageReader::open(input_path[i]).unwrap().decode().unwrap();
        
        let start = SystemTime::now().duration_since(UNIX_EPOCH).expect(E_TIME);
        let (output_image, ostring) = p_image::posterize_image(&img, k); //.expect("Failed to open image")

        println!("Image sent in {:?}ms", SystemTime::now().duration_since(UNIX_EPOCH).expect(E_TIME).as_millis() - start.as_millis());

        output_image.save("./output/posterized_image".to_string()+&i.to_string()+".png").expect("Failed to save the image");
        file.write_all(ostring.as_bytes())?;
    }
    

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
