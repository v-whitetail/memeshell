#![allow(unused, dead_code)]

use std::fs::read_dir;
use std::fs::read_to_string;
use std::path::PathBuf;
use rand::prelude::*;

fn main() {
    let path = PathBuf::new()
        .join("/")
        .join("home")
        .join("v")
        .join(".config")
        .join("memeshell");
    let len = path
        .read_dir()
        .unwrap()
        .into_iter()
        .count();
    let rng = random::<usize>() % len;
    let meme = path
        .read_dir()
        .unwrap()
        .into_iter()
        .skip(rng)
        .next()
        .unwrap()
        .and_then( |item| read_to_string(item.path()) )
        .unwrap();
    println!("{meme}");
}
