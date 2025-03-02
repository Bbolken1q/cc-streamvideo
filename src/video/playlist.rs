use std::io::Read;
use m3u8_rs::{MasterPlaylist, MediaPlaylist, Playlist};
use reqwest;

#[path ="../video/video.rs"]
mod video;
use video::get_frames;

pub async fn return_playlist(client: reqwest::Client, url: &str) {
    let binding = get_file(&client, &(url.to_owned() + "/index-f1-v1-a1.m3u8")).await;
    let mut file = binding.as_bytes();
    let mut bytes: Vec<u8> = Vec::new();
    file.read_to_end(&mut bytes).unwrap();

    let mut master: Option<MasterPlaylist> = None;
    let mut media: Option<MediaPlaylist> = None;

    match m3u8_rs::parse_playlist(&bytes) {
        Result::Ok((_i, Playlist::MasterPlaylist(pl))) => { _ = Some(pl) },
        Result::Ok((_i, Playlist::MediaPlaylist(pl))) => { media = Some(pl)} ,
        Result::Err(e) =>  panic!("Parsing error: \n{}", e),
    }

    println!("{:?}", media.clone().expect("\"media\" does not exist").segments.len());

    for var in media.expect("Master does not exist").segments {
        let mut link = String::new(); 
        

        if var.uri.contains("http") {
            link = var.uri.clone(); // uri is a link
        }
        else {
            link = url.to_owned() +"/"+ &var.uri.to_string() // not a link
        }

        if link.ends_with("js") || link.ends_with("ts") {
            println!("{}", &var.uri);
            let binding = get_file(&client, &link).await;
            let mut file = binding.as_bytes();
            
            let _ = get_frames(&file);
        }
        // let mut file = std::fs::File::open("./input/".to_owned() + &var.uri.to_string()).unwrap();
    }
}


async fn get_file(client: &reqwest::Client, url: &str) -> String {
    let text = client.get(url).send().await.expect("request failed").text().await.expect("body invalid");
    return text;
}