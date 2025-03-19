use tokio::net::TcpListener;
use tokio_tungstenite::tungstenite::Utf8Bytes;
use tokio_tungstenite::{tungstenite::protocol::Message, accept_async};
use anyhow::Result;
use futures_util::SinkExt;
use std::env;
use std::collections::VecDeque;
use std::time::Duration;
use itertools::Itertools;

#[path ="./video/playlist.rs"]
mod playlist;
use playlist::{return_playlist, get_file};

#[path ="./video/video.rs"]
mod video;
use video::get_frames;

#[path ="./posterization/sorting.rs"]
mod sorting;
pub mod pixelgroup;
#[allow(unused_imports)]
use sorting::{lightness_linear, lightness_unweighted, lightness, hue, hilbert_wrapper, hilbert_lookup, fill_lookup};

#[path ="./sort.rs"]
mod sort;
use sort::merge_sort;

mod frame;
use frame::frame::Frame;

mod qpixel; 

mod p_image;

static mut HILBERT_VALUES: Vec<Vec<Vec<u32>>> = Vec::new();

static mut FRAMES: VecDeque<Frame> = VecDeque::new();
static mut IS_EOF: bool = false;

fn get_string_index(value: &&str) -> f32 {
    // println!("{}", value.split("-").collect::<Vec<&str>>()[value.split("-").collect::<Vec<&str>>().len()-4]);
    // println!("{}", value.filename);
    value.split("-").collect::<Vec<&str>>()[value.split("-").collect::<Vec<&str>>().len()-4].parse().expect("Error parsing to float")
}

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
    
    let client = reqwest::Client::new();
    let mut frame_index: usize = 0;
    let playlist = return_playlist(&client, base_url).await;

    // let playlist: Vec<Link>  = playlist.iter().map(|link| Link { full: link, filename: link.split("/").collect::<Vec<&str>>()[link.split("/").collect::<Vec<&str>>().len() - 1]}).collect();

    // let url_length = playlist[0].split("/").collect::<Vec<&str>>().len();
    let links: Vec<&str> = playlist.iter().map(|s| s.as_str()).collect();

    merge_sort(&links, &get_string_index);

    let links = links.iter().unique();
    let mut frames: String = "".to_string();

    tokio::spawn(async { start_connection(listener).await });

    for link in links {
        let file = get_file(&client, &link).await;
        let images = get_frames(&(file.as_slice()), &mut frame_index).unwrap();
        println!("length: {}", images.len());
        for (_, element) in images {
            frames = frames + "=" + &element;
        }

        // println!("test");
        #[allow(static_mut_refs)]
        unsafe { FRAMES.push_back(Frame::new(&frames)); }

        frames = "".to_string();
    }

    unsafe { IS_EOF = true; }

    // let _ = get_frames("./input/test.ts");

    Ok(())
}

#[allow(dead_code)]
async fn handle_connection(stream: tokio::net::TcpStream) -> Result<()> {
    let mut ws_stream = accept_async(stream).await?;
    println!("CC client connected");
    ws_stream.send(Message::Text(Utf8Bytes::from_static("24"))).await?;

    #[allow(static_mut_refs)]
    while unsafe { FRAMES.len() > 0 || !IS_EOF } {
        if unsafe { FRAMES.len() > 0 } { 
            ws_stream.send(Message::text(unsafe { FRAMES.pop_front().unwrap().get_video() })).await?;
            // tokio::time::sleep(Duration::from_millis(1000/24)).await;
        }
    }

    ws_stream.send(Message::Text(Utf8Bytes::from_static("eof"))).await?;

    /* while let Some(msg) = ws_stream.next().await {
        let msg = msg?;
        if msg.is_text() {
            let received_text = msg.to_text()?;
            println!("Received message: {}", received_text);
            ws_stream.send(Message::Text(received_text.to_string())).await?;
        }
    } */

    Ok(())
}

async fn start_connection(listener: TcpListener) {
    while let Ok((stream, _)) = listener.accept().await {
        tokio::spawn(handle_connection(stream));
    }
}   


// async fn spawn_ws(listener: TcpListener) -> impl Future<Output = ()> {
//     while let Ok((stream, _)) = listener.accept().await {
//         tokio::spawn(handle_connection(stream));
//     }
// }