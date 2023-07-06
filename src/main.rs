#![allow(unused)]

use std::io;
use rand::Rng;
use std::io::{Write, BufReader, BufRead, ErrorKind};
use std::fs::File;
use std::cmp::Ordering;
fn main() {
    println!("Whats your Name:");
    
    let mut name=String::new();
    let greeting="Nice to meet you";

    io::stdin()

    .read_line(&mut name)
    .expect("Didnt receive input");

    println!("Hello {}! {}!",name.trim_end(), greeting);
}
