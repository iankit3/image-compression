mod utils;

use wasm_bindgen::prelude::*;
use js_sys::Int8Array;
use image::{ImageBuffer, RgbImage, imageops};

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern {
    fn alert(s: &str);
    fn jsLog(s: &str);
}

#[wasm_bindgen]
pub fn greet() {
    jsLog("Hello, image-compression!");
}

#[wasm_bindgen]
pub fn parse_buffer(buf : Int8Array) -> Vec<i8>{
    println!("{:?}", buf);
    let buffer: Vec<i8> = buf.to_vec();

    let mut buffer_u8: Vec<u8> = Vec::new(); 
    for elem in (&buffer).iter() {
        buffer_u8.push(*elem as u8);
    }
    //buffer_u8.len() / 8

    let mut img_buf: RgbImage = ImageBuffer::from_raw(100, 100, buffer_u8).unwrap();    
    
    //println!("{:?}", img_buf);
    //let subimg = imageops::crop(&mut img_buf, 0, 0, 100, 55);
    //let subimg = imageops::blur(&mut img_buf, 0.6);
    
    let out: Vec<u8> = imageops::blur(&mut img_buf, 1.0).to_vec();

    let mut out_i8: Vec<i8> = Vec::new();
    
    for elem in out.iter(){
        out_i8.push(*elem as i8);
    }

    out_i8

    //subimg.to_image().into_vec()
}


