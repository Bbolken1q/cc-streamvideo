#[path ="../sort.rs"]
mod sort;
use sort::merge_sort;

use std::{collections::HashMap, sync::LazyLock};
use palette::rgb::Rgb;

use colors_transform::Rgb as col_rgb;

use crate::{pixelgroup::pixel_group::PixelGroup, qpixel::qpixel::Qpixel};

#[path="../posterization/find_distance.rs"]
mod find_distance;
use find_distance::get_position;

static CHARSET: LazyLock<HashMap<u8, &str>> = std::sync::LazyLock::new(||HashMap::from([
    (0b00000000 as u8, "0x80"),
    (0b00100000 as u8, "0x81"),
    (0b00010000 as u8, "0x82"),
    (0b00110000 as u8, "0x83"),
    (0b00001000 as u8, "0x84"),
    (0b00101000 as u8, "0x85"),
    (0b00011000 as u8, "0x86"),
    (0b00111000 as u8, "0x87"),
    (0b00000100 as u8, "0x88"),
    (0b00100100 as u8, "0x89"),
    (0b00010100 as u8, "0x8a"),
    (0b00110100 as u8, "0x8b"),
    (0b00001100 as u8, "0x8c"),
    (0b00101100 as u8, "0x8d"),
    (0b00011100 as u8, "0x8e"),
    (0b00111100 as u8, "0x8f"),
    (0b00000010 as u8, "0x90"),
    (0b00100010 as u8, "0x91"),
    (0b00010010 as u8, "0x92"),
    (0b00110010 as u8, "0x93"),
    (0b00001010 as u8, "0x94"),
    (0b00101010 as u8, "0x95"),
    (0b00011010 as u8, "0x96"),
    (0b00111010 as u8, "0x97"),
    (0b00000110 as u8, "0x98"),
    (0b00100110 as u8, "0x99"),
    (0b00010110 as u8, "0x9a"),
    (0b00110110 as u8, "0x9b"),
    (0b00001110 as u8, "0x9c"),
    (0b00101110 as u8, "0x9d"),
    (0b00011110 as u8, "0x9e"),
    (0b00111110 as u8, "0x9f"),
    
]));

fn rgb_to_u8(pixel: Rgb) -> (u8, u8, u8) {
    return (pixel.red as u8, pixel.green as u8, pixel.blue as u8);
}

fn remove_first(value: &str) -> &str {
    let mut chars = value.chars();
    chars.next();
    return chars.as_str();
}

pub fn get_pixel_groups(pixels: Vec<Rgb>, centroids: Vec<Qpixel>) -> (Vec<PixelGroup>, String) {
    let mut groups: Vec<PixelGroup> = Vec::new();
    let mut output_string: String = "1=".to_string();
    for color in &centroids {
        output_string += "0x";
        output_string += remove_first(&col_rgb::from(color.color.red, color.color.red, color.color.red).to_css_hex_string());
        output_string += "|";
    }
    output_string += "=";
    for i in 0..(pixels.len()/6) {
        let mut  group: PixelGroup = PixelGroup::new([
            pixels[((i as f32/160.0).floor() * 960.0) as usize + 000 + (i*2)%320 + 0],
            pixels[((i as f32/160.0).floor() * 960.0) as usize + 000 + (i*2)%320 + 1],
            pixels[((i as f32/160.0).floor() * 960.0) as usize + 320 + (i*2)%320 + 0],
            pixels[((i as f32/160.0).floor() * 960.0) as usize + 320 + (i*2)%320 + 1],
            pixels[((i as f32/160.0).floor() * 960.0) as usize + 640 + (i*2)%320 + 0],
            pixels[((i as f32/160.0).floor() * 960.0) as usize + 640 + (i*2)%320 + 1]
        ]);
        posterize_group(&mut group, &centroids);
        
        // output_string += &("-1&1*".to_owned() + group.character.as_str() + "," + group.c1.to_string().as_str() + "," + group.c2.to_string().as_str());
        
        groups.push(group); 
    }

    for x in 0..60 {
        output_string += "-";
        for j in 0..160 {
            output_string += &("|1&1*".to_owned() + groups[x*160 + j].character.as_str() + "," + groups[x*160+j].c1.to_string().as_str() + "," + groups[x*160+j].c2.to_string().as_str());
        } 
    }

    return (groups, output_string);
}

fn posterize_group(group: &mut PixelGroup, centroids: &Vec<Qpixel>) {
    let group_sorted = merge_sort(&group.pixels);

    for i in 0..6 {
        let mut hues:Vec<f32> = Vec::with_capacity(centroids.len());
        for color in centroids {
            hues.push(color.hue);
        }
        group.c1 =  get_position(&hues, group_sorted[0].hue);
        group.c2 =  get_position(&hues, group_sorted[0].hue);

        if (group.pixels[i].hue - group_sorted[0].hue).abs() <= (group.pixels[i].hue - group_sorted[5].hue).abs() {
            group.pixels[i] = group_sorted[0];
            group.structure = group.structure << 1;
            group.structure += 1;

        } else {
            group.pixels[i] = group_sorted[5];
            group.structure = group.structure << 1
        }
    }

    match CHARSET.get(&group.structure) {
        Some(char) => {
            group.character = char.to_string();
            // println!("{}, {:08b}", char, group.structure);
        }
        None => {
            match CHARSET.get(&!group.structure) {
                Some(char) => {
                    group.character = char.to_string();
                    let buf = group.c1;
                    group.c1 = group.c2;
                    group.c2 = buf;
                    // println!("{}, {:08b}", char, group.structure);
                }
                None => {}
            }
        }
    }

    // println!("{:06b}", group.structure);
}

pub fn get_u8_pixels(_pixels: Vec<PixelGroup>) -> Vec<u8> {
    let mut pixels: Vec<Rgb> = vec![Rgb::new(0.0, 0.0, 0.0); 320*180];
    let width = 160;
    let height = 60;

    for i in 0..(width*height) {
        pixels[((i as f32/160.0).floor() * 960.0) as usize + 000 + (i*2)%320 + 0] = _pixels[i].pixels[0].color;
        pixels[((i as f32/160.0).floor() * 960.0) as usize + 000 + (i*2)%320 + 1] = _pixels[i].pixels[1].color;
        pixels[((i as f32/160.0).floor() * 960.0) as usize + 320 + (i*2)%320 + 0] = _pixels[i].pixels[2].color;
        pixels[((i as f32/160.0).floor() * 960.0) as usize + 320 + (i*2)%320 + 1] = _pixels[i].pixels[3].color;
        pixels[((i as f32/160.0).floor() * 960.0) as usize + 640 + (i*2)%320 + 0] = _pixels[i].pixels[4].color;
        pixels[((i as f32/160.0).floor() * 960.0) as usize + 640 + (i*2)%320 + 1] = _pixels[i].pixels[5].color;
    }

    let mut colors: Vec<u8> = Vec::new();

    for i in 0.. pixels.len() {
        let (r, g, b) = rgb_to_u8(pixels[i]);
        colors.push(r);
        colors.push(g);
        colors.push(b);
    }

    return colors;
}



// posortuj lightness -> sprawdź czy blizej do najmniejszego czy największego -> przyporządkuj -> uśrednij -> profit