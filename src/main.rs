#![allow(unused_imports)]
use rs_samples::builder::string::StringBuilder;
use rs_samples::roman::roman;

fn main() {
    let mut sb = StringBuilder::default();
    let world = "World".to_string();
    sb = sb.add("Hello")
        .add(' ')
        .add('a')
        .add(' ')
        .add(world);
    println!("{}", sb.string());

    /*
    let v = roman::to_int("VII");
    println!("{:?}", v);

    let s = roman::to_roman(v.unwrap());
    println!("{:?}", s);
     */
}
