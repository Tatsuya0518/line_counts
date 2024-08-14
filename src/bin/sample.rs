use std::fs::File;
use std::io::{BufReader, BufRead};


fn main(){
    println!("Hello World!!")   
}

pub fn open_file(filename: &str) -> (Vec<f32>, Vec<f32>){
    main();
    let f = File::open(filename).unwrap();
    let buf = BufReader::new(f);
    let mut x: Vec<f32> = Vec::new();
    let mut y: Vec<f32> = Vec::new();
    for line in buf.lines() {
        let l: &str = &line.unwrap();
        let p: Vec<&str> = l.trim().split(" ").collect();
        x.push(p[0].parse().unwrap());
        y.push(p[1].parse().unwrap());
    }
    (x, y)
}