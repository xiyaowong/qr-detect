use std::{env, fs, iter::FromIterator};

use image::EncodableLayout;
use rqrr::PreparedImage;

fn detect(buf: &[u8]) {
    let img = image::load_from_memory(&buf).unwrap().to_luma8();
    let mut img = PreparedImage::prepare(img);
    let grids = img.detect_grids();
    let (_, content) = grids[0].decode().unwrap();
    print!("{}", content);
}

fn main() {
    let args = env::args();
    if args.len() < 2 {
        panic!("wrong arguments");
    }
    let args = Vec::from_iter(args.into_iter());
    let data = &args[1];

    // url
    if data.starts_with("http") {
        let res = reqwest::blocking::get(data).unwrap();
        let content = res.bytes().unwrap();
        return detect(content.as_bytes());
    }

    // base64
    if let Ok(buf) = base64::decode(data) {
        return detect(buf.as_bytes());
    };

    // local path
    let buf = fs::read(data).unwrap();
    return detect(buf.as_bytes());
}
