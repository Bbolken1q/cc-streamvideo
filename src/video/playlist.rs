use std::{fmt::Error, io::Read};
use image::EncodableLayout;
use m3u8_rs::{MasterPlaylist, MediaPlaylist, Playlist};
use reqwest;

#[path ="../video/video.rs"]
mod video;
use video::get_frames;

pub async fn return_playlist(client: reqwest::Client, url: &str) {
    let file = get_file(&client, &(url.to_owned() + "/master.m3u8")).await;
    let mut bytes: Vec<u8> = Vec::new();
    file.as_bytes().read_to_end(&mut bytes).unwrap();

    let (master, _ ) = get_m3u8(&file).unwrap();

    let mut frame_index: usize = 0;

    for variant in master.expect("\"media\" does not exist").variants
    {   

        let file = get_file(&client, &(url.to_owned() + "/index-f1-v1-a1.m3u8")).await;
        let mut bytes: Vec<u8> = Vec::new();
        file.as_bytes().read_to_end(&mut bytes).unwrap();


        let (_, media) = get_m3u8(&file).unwrap();

        println!("{:?}", media.clone().expect("\"media\" does not exist").segments.len());

        for var in media.expect("\"media\" does not exist").segments {
            let link; 

            if var.uri.contains("http") {
                link = var.uri.clone(); // uri is a link
            }
            else {
                link = url.to_owned() +"/"+ &var.uri.to_string() // not a link
            }

            if link.ends_with("js") || link.ends_with("ts") {
                println!("{}", &var.uri);
                let file = get_file(&client, &link).await;

                let _ = get_frames(&(file.as_slice()), &mut frame_index);
            }
            // let mut file = std::fs::File::open("./input/".to_owned() + &var.uri.to_string()).unwrap();
        }
    }
}

async fn get_file(client: &reqwest::Client, url: &str) -> Vec<u8> {
    let buf = client.get(url).send().await.expect("request failed").bytes().await.unwrap();
    return buf.as_bytes().to_vec();
}

fn get_m3u8(bytes: &[u8]) -> Result<(Option<MasterPlaylist>, Option<MediaPlaylist>), Error> {
    let mut master: Option<MasterPlaylist> = None;
    let mut media: Option<MediaPlaylist> = None;

    match m3u8_rs::parse_playlist(bytes) {
        Result::Ok((_, Playlist::MasterPlaylist(pl))) => { master = Some(pl).or(None); },
        Result::Ok((_, Playlist::MediaPlaylist(pl))) => { media = Some(pl).or(None); } ,
        Result::Err(e) =>  panic!("Parsing error: \n{}", e),
    }

    if master != None {
        return Ok((master, None))
    } else if media != None {
        return Ok((None, media))
    } else {
        return Err(Error)
    }
    
}