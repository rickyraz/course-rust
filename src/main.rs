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
   struct TreeNode<T> {
    pub left : Option<Box<TreeNode<T> > >,
    pub right : Option<Box<TreeNode<T> > >,
    pub key : T,
    }

    impl <T> TreeNode<T>{
        pub fn new(key:T) -> Self {
            TreeNode { left: None, right: None, key,
            }
        }
        pub fn left(mut self, node:TreeNode<T>) -> Self {
            self.left = Some(Box::new(node)) ;
            self
        }
        pub fn right(mut self, node:TreeNode<T>) -> Self {
            self.right = Some(Box::new(node)) ;
            self
        }
    }
    
}



fn closures_in_fn(){
    fn use_func<T>(a : i32, b : i32, func : T) -> i32
    where T : Fn(i32, i32) -> i32 {
        func(a, b)
    }

    let sum = | a:i32, b:i32 | a + b ;
    let prod = | a:i32, b:i32 | a * b ;
    println!("22 + 6 = {}", use_func(22, 6, sum));
    println!("22 * 6 = {}", use_func(22, 6, prod));
}
fn closures() {
    // basic
    let can_vote = |age: i32| age >= 18;
    println!("Can Vote : {}", can_vote(8));

    //more advanced
    let mut samp1 = 5;
    let print_var = || println!("samp1 = {}", samp1);
    print_var();

    samp1 = 10;

    let mut change_var = || samp1 += 1;
    change_var();
    println!("samp1 = {}", samp1);

    samp1 = 10;
    println!("samp1 = {}", samp1);
}
fn iter() {
    let mut arr = [1, 2, 3, 4, 5];
    for val in arr.iter() {
        println!("{}", val);
    }

    let mut iter1 = arr.iter();
    println!("1st {:?}", iter1.next())
}
fn error_handling() {
    // create file, read file, implementing Input & Output Mechanism
    let path = "lines.txt";
    let output = File::create(path);
    let mut output = match output {
        Ok(file) => file,
        Err(error) => panic!("Problem creating file : {:?}", error),
    };

    // Write is trait, you must use the macro ->> write!
    write!(output, "Just some\nRandom Words").expect("Fialed to write to file");

    // unwrap ignores the result , and gives us the output
    let input = File::open(path).unwrap();
    let buffered = BufReader::new(input);
    for line in buffered.lines() {
        println!("{}", line.unwrap())
    }

    // catch spesific errors (using ErrorKind)
    let mut output2 = File::create("rand.txt");
    let output2 = match output2 {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("rand.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Cant create file {:?} for some unknown reason", error),
            },
            _other_error => panic!("Problem opening file : {:?}", error),
        },
    };
}
