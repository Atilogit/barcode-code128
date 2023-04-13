use std::{env::args, io::BufRead};

use barcode_code128::encode_str;

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
