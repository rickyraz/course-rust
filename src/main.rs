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

fn main() {
    println!("Hello World")
}
