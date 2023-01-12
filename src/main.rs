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
// Hash-Map Data Structure
use std::collections::HashMap;

// Using module restaurant
mod restaurant;
use crate::restaurant::order_food;

fn main() {
    order_food();
    users();
}

fn nyoba2() {
    let v = vec![1, 2, 478, 4, 5, 6, 0612];

    let third: &i32 = &v[2];
    println!("The third element is {}", third);

    let sixth: Option<&i32> = v.get(6);
    match sixth {
        Some(sixth) => println!("The six element is {}", sixth),
        None => println!("There is no six element."),
    }

    //---

    let mut vc = vec![100, 32, 57];
    for i in &mut vc {
        *i += 50;
        println!("{}", i);
    }

    //---

    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];

    //---
}

fn users() {
    struct User {
        active: bool,
        username: String,
        email: String,
        sign_in_count: u64,
    }

    //--

    let condition = false;
    let number = if condition { 5 } else { 6 };

    println!("The value of number is: {number}");

    let mut user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("rickkzz"),
        active: true,
        sign_in_count: 1,
    };

    // let user2 = User {
    //     active: user1.active,
    //     username: user1.username,
    //     email: String::from("another@example.com"),
    //     sign_in_count: user1.sign_in_count,
    // };

    user1.username = String::from("azharzz");

    //  use the rest of the values from user1
    let user3 = User {
        email: String::from("rickr@example.com"),
        ..user1
    };

    // println!("hi {}", user1.username);
    println!("hi {}", user3.username);

    //-- unimplement build user !!
    fn build_user(email: String, username: String) -> User {
        User {
            email,
            username,
            active: true,
            sign_in_count: 1,
        }
    }
}
