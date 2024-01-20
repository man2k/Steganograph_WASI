use std::env;
use steganography::decoder::*;
use steganography::encoder::*;
use steganography::util::*;

pub fn steganograph(img_path: String, data: String, out_path: String) -> Result<(), String> {
    let finaldata = str_to_bytes(&data);
    // eprintln!("{:?}", finaldata);
    let destination_image = file_as_dynamic_image(img_path);
    let enc = Encoder::new(&finaldata, destination_image);
    let result = enc.encode_alpha();
    save_image_buffer(result, out_path);
    Ok(())
}

pub fn desteganograph(img_path: String) -> Result<String, String> {
    let encoded_image = file_as_image_buffer(img_path);
    let dec = Decoder::new(encoded_image);
    let out_buffer = dec.decode_alpha();

    let clean_buffer: Vec<u8> = out_buffer.into_iter().filter(|b| *b != 0xff_u8).collect();
    let msg = bytes_to_str(clean_buffer.as_slice());
    eprintln!("{:?}", msg);
    return Ok(msg.to_string());
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let program = args[0].clone();

    if args.len() < 2
        && args[1] != "mank-wasi-rust.wasm"
        && args[1] != "steg"
        && args[1] != "unsteg"
    {
        eprintln!("usage: {} steg <image path> <text> <output path>", program);
        eprintln!("usage: {} unsteg <image path>", program);
        return;
    }

    if &args[1] == "steg" {
        let _ = steganograph(args[2].to_owned(), args[3].to_owned(), args[4].to_owned());
    }
    if &args[1] == "unsteg" {
        let msg = desteganograph(args[2].to_owned()).unwrap();
        eprintln!("Text Revealed -> {:?}", msg);
    }
}
