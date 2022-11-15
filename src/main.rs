#![allow(unused)]

use rand::Rng;
// Comparing values
use std::cmp::Ordering;
// Create, Read, Write file
use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader, ErrorKind, Write};
// Add trait
use std::ops::Add;
// Hash Map Data Structure
use std::collections::HashMap;

// Using module restaurant
mod restaurant;
use crate::restaurant::order_food;
fn five() -> i32 {
    5
}
// if add 'semicolon' at the end of 1 it will be statement not expression
fn plus_one(x: i32) -> i32 {
    x + 1
}

fn main() {
    let condition = false;
    let number = if condition { 5 } else { 6 };

    println!("The value of number is: {number}");
}
