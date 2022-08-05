#![allow(unused)]

use rand::Rng;
use std::cmp::Ordering;
use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader, ErrorKind, Write};
// Add trait
use std::ops::Add;
// Hash Map
use std::collections::HashMap;

mod restaurant;
use crate::restaurant::order_food;


fn main() {
    // create file, read file, implementing Input & Output Mechanism
    let path = "lines.txt";
    let output = File::create(path);
    let mut output = match output {
        Ok(file) => file,
        Err(error) => 
            panic!("Problem creating file : {:?}", error )
    };
    
    // Write is trait, you must use the macro ->> write!
    write!(output, "Just some\nRandom Words").expect("Fialed to write to file");

    // unwrap ignores the result , and gives us the output
    let input = File::open(path).unwrap();
    let buffered = BufReader::new(input);
    for line in buffered.lines(){
        println!("{}", line.unwrap() )
    }

    // catch spesific errors (using ErrorKind)
    let output2 = File::create("rand.txt");
    let output2 = match output2 {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("rand.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Cant create file {:?} for some unknown reason", error)
            },
            _other_error => panic!("Problem opening file : {:?}", error)
        },
    };
}
