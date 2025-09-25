use futures::TryStreamExt;
use tokio::net::TcpListener;
use tokio_tungstenite::tungstenite::Utf8Bytes;
use tokio_tungstenite::{tungstenite::protocol::Message, accept_async};
use anyhow::Result;
use futures_util::SinkExt;
use std::env;
use std::collections::VecDeque;
use std::time::Duration;
use itertools::Itertools;

use std::fs::File;
use std::io::prelude::*;

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

    let file_type: bool; // check if the input is a file or a link

    let args: Vec<String> = env::args().collect();
    if args.len() == 1 {
        println!("You have not passed an url or a filename");
        std::process::exit(0);
    }
    match &args[1].as_str() {
        &"-l"|&"--link" => {file_type = false},
        &"-f"|&"--file" => {file_type = true},
        _ => {
            println!("You have passed an unknown argument: {}", &args[1]);
            std::process::exit(0);
        } 
    }
    let base_filename: &str = &args[2]; 
    // println!("playing video from {}", base_url);

    let addr = "127.0.0.1:3000".to_string();
    #[allow(unused_variables)]
    let listener = TcpListener::bind(&addr).await?;
    println!("WebSocket server started on ws://{}", addr);

    fill_lookup(); // create the lookup table 
    let mut frame_index: usize = 0;
    let mut frames: String = "".to_string();
    
    let mut handles = vec![];

    handles.push(tokio::spawn(async { start_connection(listener).await }));


    if(!file_type) {
        let client = reqwest::Client::new();
        let playlist = return_playlist(&client, base_filename).await;

        let links: Vec<&str> = playlist.iter().map(|s| s.as_str()).collect();

        merge_sort(&links, &get_string_index);

        let links = links.iter().unique();
        
        // tokio::spawn(async { start_connection(listener).await });


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
    } else {
        let file = std::fs::read(base_filename).expect("Failed to read the file");
        let images = get_frames(&(file.as_slice()), &mut frame_index).unwrap(); // not good, needs a fucking terabyte of ram. Ideally would like to split the file into chunks and process them one by one
        for (_, element) in images {
            // frames = frames + "=" + &element;

            #[allow(static_mut_refs)]
        unsafe { FRAMES.push_back(Frame::new(&element)); }
        }

        

        println!("pushed frames");

        println!("{}", unsafe { FRAMES.len()});

        frames = "".to_string();
    }

    unsafe { IS_EOF = true; }

    futures::future::join_all(handles).await;
    
    // let mut file = File::create("./output/output.txt")?;

    // for frame in unsafe { FRAMES.iter() } {
    //     file.write_all((frame.get_video() + "\n").as_bytes())?;
    // } 

    // println!("finished");

    // let _ = get_frames("./input/test.ts");

    Ok(())
}

#[allow(dead_code)]
async fn handle_connection(stream: tokio::net::TcpStream) -> Result<()> {
    let mut ws_stream = accept_async(stream).await?;
    println!("CC client connected");
    ws_stream.send(Message::Text(Utf8Bytes::from_static("24"))).await?;
    println!("sent fps");

    // let test: String = "test message".to_string();
    // ws_stream.send(Message::Text(Utf8Bytes::from(&test))).await?;<s

    #[allow(static_mut_refs)]
    while unsafe { FRAMES.len() > 0 || !IS_EOF } {
        // println!("is this even running");
        tokio::time::sleep(Duration::from_millis(1000/10)).await; // what the actual fuck 
        while unsafe { FRAMES.len() > 0 } { 
            // println!("length not 0");
            let message = unsafe { FRAMES.pop_front().unwrap().get_video() };
            // println!("{}", message);
            // println!("sending message");
            if(message.len() > 0) {
                ws_stream.send(Message::Text(Utf8Bytes::from(&message))).await?;
            }

            // tokio::time::sleep(Duration::from_millis(1000/24)).await;
        }
    }

    println!("eof");

    ws_stream.send(Message::Text(Utf8Bytes::from_static("eof"))).await?;

     while let Some(msg) = ws_stream.try_next().await.unwrap() {
        let msg = msg;
        if msg.is_text() {
            let received_text = msg.to_text()?;
            println!("Received message: {}", received_text);
            ws_stream.send(Message::Text(Utf8Bytes::from(received_text.to_string()))).await?;
        }
    } 

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