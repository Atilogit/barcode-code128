use std::{env::args, io::BufRead};

use image::GenericImageView;
use rxing::{
    common::BitArray,
    oned::{Code128Reader, Code128Writer, OneDReader, OneDimensionalCodeWriter},
};

fn main() {
    if args().count() == 2 {
        let arg = args().nth(1).unwrap();
        let code = encode_str(&arg);
        println!("{code}");
        println!("{}", code.len());
    } else {
        for line in std::io::stdin().lock().lines() {
            let code = encode_str(&line.unwrap());
            println!("{code}");
            println!("{}", code.len());
        }
    }
}

fn encode_str(text: &str) -> String {
    let encoded = Code128Writer::default().encode_oned(text).unwrap();
    encoded
        .into_iter()
        .map(|v| if v { '|' } else { ' ' })
        .collect()
}

fn decode_str(code: &str) -> String {
    let mut arr = BitArray::new();
    for c in code.chars() {
        arr.appendBit(c == '|');
    }
    let res = Code128Reader::default()
        .decode_row(0, &arr, &Default::default())
        .unwrap();
    res.to_string()
}

fn read_img(path: &str) -> String {
    let img = image::open(path).expect("Image not found");
    let mut code = String::new();
    for x in (0..img.width()).step_by(2) {
        let pix = img.get_pixel(x, 0);
        if pix.0[0] == 0 {
            code.push('|');
        } else {
            code.push(' ');
        }
    }
    code
}

#[test]
fn test_codes() {
    let tests=[
        ("NSCFA2305E99ARRRRRRRRRRRY", "|| |  |    | |||   || || ||| |   |   |   || |   ||   | | |   ||   | ||| |||| ||| || ||| |   |  ||  | |||| ||| |   || |   |||  | ||  |||  | ||  | |   ||   ||   | ||| ||   | ||| ||   | ||| ||   | ||| ||   | ||| ||   | ||| ||   | ||| ||   | ||| ||   | ||| ||   | ||| ||   | ||| ||| || |   |  | ||    ||   ||| | ||"),
        ("TSCFA2306E7AY0BFD6L",       "|| |  |    || |||   | || ||| |   |   |   || |   ||   | | |   ||   | ||| |||| ||| || ||| |  ||  |   | |||| ||| |   || |   ||| || ||| | |   ||   ||| || |   |  ||| ||  |   | ||   |   ||   | | ||   |   ||  ||| |  |   || ||| |  ||| ||  ||   ||| | ||"),
        ("LSCFA2305E99ARRRRRRRRRRRE", "|| |  |    |   || ||| || ||| |   |   |   || |   ||   | | |   ||   | ||| |||| ||| || ||| |   |  ||  | |||| ||| |   || |   |||  | ||  |||  | ||  | |   ||   ||   | ||| ||   | ||| ||   | ||| ||   | ||| ||   | ||| ||   | ||| ||   | ||| ||   | ||| ||   | ||| ||   | ||| ||   | ||| |   || |   ||    | |  ||   ||| | ||")
    ];

    for (text, code) in tests {
        assert_eq!(text, decode_str(code));
        assert_eq!(encode_str(text), code);
        assert_eq!(decode_str(&encode_str(text)), text);
    }
}
