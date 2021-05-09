extern crate miniz_oxide;

use std::str;
use std::env;
use miniz_oxide::inflate::decompress_to_vec;
use miniz_oxide::deflate::compress_to_vec;

fn main() {
    let args: Vec<String> = env::args().collect();
    let val = args[1].as_bytes();
    let compressed = compress_to_vec(val, 6);
    let decompressed = decompress_to_vec(compressed.as_slice()).expect("Failed to decompress!");


    println!("{}", str::from_utf8(&decompressed).unwrap());

}
