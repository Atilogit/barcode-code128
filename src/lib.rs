use std::ffi::{CStr, CString};

use image::GenericImageView;
use rxing::{
    common::BitArray,
    oned::{Code128Reader, Code128Writer, OneDReader, OneDimensionalCodeWriter},
};

/// # Safety
#[no_mangle]
pub unsafe extern "C" fn encode_str_ffi(text: *const i8) -> *const i8 {
    let text = unsafe { CStr::from_ptr(text) };
    let result = encode_str(text.to_str().unwrap());
    let result = CString::new(result.as_bytes()).unwrap();
    let ptr = result.as_ptr();
    std::mem::forget(result);
    ptr
}

pub fn encode_str(text: &str) -> String {
    let encoded = Code128Writer::default().encode_oned(text).unwrap();
    encoded
        .into_iter()
        .map(|v| if v { '|' } else { ' ' })
        .collect()
}

pub fn decode_str(code: &str) -> String {
    let mut arr = BitArray::new();
    for c in code.chars() {
        arr.appendBit(c == '|');
    }
    let res = Code128Reader::default()
        .decode_row(0, &arr, &Default::default())
        .unwrap();
    res.to_string()
}

pub fn read_img(path: &str) -> String {
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

#[test]
fn test_ffi() {
    let text = "hallo";
    let c_text = CString::new(text.as_bytes()).unwrap();
    let result_ptr = unsafe { encode_str_ffi(c_text.as_ptr()) };
    let c_result = unsafe { CStr::from_ptr(result_ptr) };
    let result = c_result.to_string_lossy();

    assert_eq!(result, encode_str(text));
}
