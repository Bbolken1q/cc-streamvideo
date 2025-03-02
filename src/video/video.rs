extern crate ffmpeg_next as ffmpeg;
use std::{ffi::{c_char, CString}, fmt::Debug, fs::File, io::{Cursor, Read}, ptr, result::Result::Ok, str::FromStr, u8};
use ffmpeg::{format::{input, Pixel, context::{Input, destructor::*}}, media::Type, software::scaling::{context::Context, flag::Flags}, util::{frame::video::Video, error::Error}, ffi::*};
use image::{DynamicImage, RgbImage};
use std::process::Command;

use crate::p_image;

struct AVIOBuffer <'a> {
    cursor: Cursor<&'a [u8]>,
}

impl AVIOBuffer<'_> {
    unsafe extern "C" fn read_packet(opaque: *mut std::ffi::c_void, buf: *mut u8, buf_size: i32) -> i32 {
        let buffer = &mut *(opaque as *mut AVIOBuffer);
        
        // Read directly into the FFmpeg buffer instead of allocating temp_buf
    
        let slice;

        if buf != ptr::null_mut() {
            slice = std::slice::from_raw_parts_mut(buf, buf_size as usize);
        }
        else {
            return AVERROR_EOF
        }

        match buffer.cursor.read(slice) {
            Ok(0) => AVERROR_EOF, // Indicate EOF when no bytes are read
            Ok(size) => size as i32,
            Err(_) => AVERROR(EIO)
        }
    }
}

// fn create_input(file: &[u8]) -> Result<Input, Error> {
//     unsafe {

//         let avio_buffer = av_malloc(8192) as *mut u8;

//         if avio_buffer.is_null() {
//             return Err(Error::InvalidData);
//         }

//         let ctxt: *mut AVIOContext = avio_alloc_context(avio_buffer, 8192, 0, &mut avio_buffer_struct as *mut _ as *mut std::ffi::c_void, Some(AVIOBuffer::read_packet), None, None);
//         let mut ps = avformat_alloc_context();

//         (*ps).pb = ctxt;
//         (*ps).flags = AVFMT_FLAG_CUSTOM_IO;

//         return Ok(Input::wrap(ps)); 

//         // avformat_close_input(&mut ps);
//         // av_free(ctxt as *mut std::ffi::c_void);
//     }
// }

fn create_input(file: &[u8]) -> Result<Input, Error> {
    unsafe {
        let buf_sz: i32 = 8192;
        let buf = av_malloc(buf_sz as usize) as *mut u8;

        let mut avio_buffer_struct = AVIOBuffer {
            cursor: Cursor::new(file),
        };

        let avio_context = avio_alloc_context(buf, buf_sz, 0, &mut avio_buffer_struct as *mut _ as *mut std::ffi::c_void, Some(AVIOBuffer::read_packet), None, None);

        let mut context = avformat_alloc_context();

        (*context).pb = avio_context;
        (*context).flags |= AVFMT_FLAG_CUSTOM_IO;

        // println!("{:?}", *avio_context);

        match avformat_open_input(&mut context, CString::from_str("").unwrap().as_ptr(), ptr::null_mut(), ptr::null_mut()) {
            0 => match avformat_find_stream_info(context, ptr::null_mut()) {
                r if r >= 0 => Ok(Input::wrap(context)),
                e => {
                    avformat_close_input(&mut context);
                    Err(Error::from(e))
                }
            },

            e => {Err(Error::from(e))},
        }
    }
}

fn receive_and_process_decoded_frames(decoder: &mut ffmpeg::decoder::Video, frame_index: &mut usize, scaler: &mut Context) -> Result<(DynamicImage, String), ffmpeg::Error> {
    let mut decoded = Video::empty();
    let mut output_image: DynamicImage = DynamicImage::new(0, 0, image::ColorType::Rgb8);
    let mut ostring: String = String::new();

    while decoder.receive_frame(&mut decoded).is_ok() {
        let mut rgb_frame = Video::empty();
        scaler.run(&decoded, &mut rgb_frame)?;
        // save_file(&rgb_frame, frame_index).unwrap();
        let image = DynamicImage::ImageRgb8(RgbImage::from_raw(rgb_frame.width(), rgb_frame.height(), rgb_frame.data(0).to_vec()).ok_or("Failed to create image").expect("Failed to create image"));
        // let _ = image.save("./frame".to_owned() + &frame_index.to_string().to_owned() + ".png");
        (output_image, ostring) = p_image::posterize_image(&image, 15);
        output_image.save("./output/posterized_image".to_string()+&frame_index.to_string()+".png").expect("Failed to save the image");
        *frame_index += 1;
        println!("{}", frame_index)
        
    }
    Ok((output_image, ostring))
}


pub fn get_frames(file: &[u8]) -> Result<(), ffmpeg::Error> {

    let mut buf: Vec<u8> = Vec::new();
    let mut f = File::open("./input/test.ts").unwrap();

    let _ = f.read_to_end(&mut buf);

    if let Ok(mut ictx) = input("./input/test.ts") { // create_input(&buf)
        let input = ictx
            .streams()
            .best(Type::Video)
            .ok_or(ffmpeg::Error::StreamNotFound)?;
        let video_stream_index = input.index();

        let context_decoder = ffmpeg::codec::context::Context::from_parameters(input.parameters())?;
        let mut decoder = context_decoder.decoder().video()?;
        // let mut decoder = input.codec().decoder().video()?;

        let mut scaler = Context::get(
            decoder.format(),
            decoder.width(),
            decoder.height(),
            Pixel::RGB24,
            decoder.width(),
            decoder.height(),
            Flags::BILINEAR,
        )?;

        let mut frame_index = 0;

        let mut receive_and_process_decoded_frames =
            |decoder: &mut ffmpeg::decoder::Video| -> Result<(), ffmpeg::Error> {
                let mut decoded = Video::empty();
                while decoder.receive_frame(&mut decoded).is_ok() {
                    let mut rgb_frame = Video::empty();
                    scaler.run(&decoded, &mut rgb_frame)?;
                    // save_file(&rgb_frame, frame_index).unwrap();

                    let image = DynamicImage::ImageRgb8(RgbImage::from_raw(rgb_frame.width(), rgb_frame.height(), rgb_frame.data(0).to_vec()).ok_or("Failed to create image").expect("Failed to create image"));
                    // let _ = image.save("./frame".to_owned() + &frame_index.to_string().to_owned() + ".png");
                    let (_output_image, ostring) = p_image::posterize_image(&image, 15);
                    // _output_image.save("./output/posterized_image".to_string()+&frame_index.to_string()+".png").expect("Failed to save the image");
                    println!("{}", frame_index);
                    frame_index += 1;
                }
                Ok(())
            };


        // println!("{:?}", ictx.packets());
        for (stream, packet) in ictx.packets() {

            if stream.index() == 0 {
                // println!("{:?}", packet.data());
            // {
                decoder.send_packet(&packet)?;
                receive_and_process_decoded_frames(&mut decoder)?;
            }
        }
        decoder.send_eof()?;
        receive_and_process_decoded_frames(&mut decoder)?;
    }

    Ok(())
}

