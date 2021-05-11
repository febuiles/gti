use flate2::read::ZlibDecoder;
use std::env;
use std::io::Read;

fn inflate(buf: Vec<u8>) {
    let mut decoder = ZlibDecoder::new(buf.as_slice());
    let mut res = String::new();

    match decoder.read_to_string(&mut res) {
        Ok(_) => {
            println!("{}", res);
        }
        Err(e) => {
            eprintln!("inflate: {:?}", e);
            std::process::exit(2);
        }
    }
}

fn main() {
    let args:Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("inflate: no file provided");
        std::process::exit(1);
    }
    match std::fs::read(&args[1]) {
        Ok(bytes) => {
            inflate(bytes);
        }
        Err(_) => {
            eprintln!("inflate: error reading file {}", args[1]);
            return;
        }
    }
}
