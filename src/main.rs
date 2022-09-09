// extern crate chardet;
// extern crate encoding;
use chardet;
use std::fs::OpenOptions;
use std::io::prelude::*;
// use encoding::DecoderTrap;
// use encoding::label::encoding_from_whatwg_label;


fn main() {
    // println!("Hello, world!");

    let filepath: String = "../sample_file.txt".to_string();

    // open text file
    let mut fh = OpenOptions::new().read(true).open(filepath).expect(
        "Could not open file",
    );
    let mut reader: Vec<u8> = Vec::new();

    // read file
    fh.read_to_end(&mut reader).expect("Could not read file");

    // detect charset of the file
    let result = chardet::detect(&reader);

    // result.0 Encode
    // result.1 Confidence
    // result.2 Language

    println!( "result, ``{:?}``", result );

}
