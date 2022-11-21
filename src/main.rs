#![allow(unused_imports)]

use std::io::Read;
use rs_samples::builder::string::{BytesCovertible, StringBuilder};
// use rs_samples::crypto::blowfish;
use rs_samples::crypto::blowfish::Blowfish;
use rs_samples::roman::roman;

fn main() {
    let bf = Blowfish::new_with_string("TEST").unwrap();
    let cipher = bf.encrypt_ecb(&"Piotr Pszczółkowski 123".as_bytes().to_vec()).unwrap();
    println!("{:x?}", cipher);
    let plain = bf.decrypt_ecb(&cipher).unwrap();
    println!("{:?}", String::from_utf8_lossy(&plain));

    println!("------------------------------------------------------");

    let x = bf.encrypt_cbc(&"Adam, Artur, Błażej, Kacpi, Nikoś".as_bytes().to_vec()).unwrap();
    println!("{:x?}", x);
    let y = bf.decrypt_cbc(&x);
    println!("{:?}", String::from_utf8_lossy(&y.unwrap()));



    /*
    let mut sb = StringBuilder::default();
    let world = "World".to_string();
    sb = sb.add("Hello")
        .add(' ')
        .add('a')
        .add(' ')
        .add(world);
    println!("{}", sb.string());

     */

    /*
    let v = roman::to_int("VII");
    println!("{:?}", v);

    let s = roman::to_roman(v.unwrap());
    println!("{:?}", s);
     */
}
