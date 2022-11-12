// ----- example basic concept2

fn greetings() {
    println!("What is yor name?");
    let mut name = String::new();
    let greeting = "Nice to meet you";
    io::stdin().read_line(&mut name);
    // .expect_err("receive input");
    println!("Hello, {}! {} ", name.trim_end(), greeting);
}

fn variables() {
    const ONE_MIL: u32 = 1_000_000;
    const PI: f64 = 3.141592;

    let age = "21";
    let mut age: u32 = age.trim().parse().expect("Age wasnt assigned a number");
    age = age + 2;

    println!(
        "When i {} years old, i hope have a ${} dollars in my bank account",
        age, ONE_MIL
    );
}

fn number() {
    println!("f32 MAX {}", f32::MAX);

    let num_1 = 1.11111111;
    let num_2 = 0.11111111111;
    println!("{}", num_1 + num_2);

    let random_num = rand::thread_rng().gen_range(1..74);
    println!("random number : {}", random_num);
}

fn matches() {
    let mut my_age = 18;

    // Ternary Operator
    let can_vote = if (my_age >= 18) { true } else { false };
    println!("Can Vote? {}", can_vote);

    // Match
    let age2 = 8;
    match age2 {
        1..=18 => println!("Important Brithday"),
        21 | 69 => println!("Important Brithday"),
        79..=i32::MAX => println!("Important Brithday"),
        // underscore will match everything else
        _ => println!("Not Important Brithday"),
    };

    // Match 2
    let my_age = 18;
    let voting_age = 18;

    match my_age.cmp(&voting_age) {
        Ordering::Less => println!("can't Vote"),
        Ordering::Greater => println!("can Vote"),
        Ordering::Equal => println!("you gained to the right voice"),
    }
}

fn arrays() {
    let arr_1 = [0, 1, 2, 3];
    println!("1st array : {}", arr_1[0]);
    println!("array length : {}", arr_1.len());

    //Looping - you decide to use any form of looping
    let arr_2 = [1, 2, 3, 4, 5, 6, 7, 8, 9];
    let mut loop_index = 0;

    // -- regular looping
    loop {
        if arr_2[loop_index] % 2 == 0 {
            loop_index += 1;
            continue;
        }
        if arr_2[loop_index] == 9 {
            break;
        }
        println!("Val : {}", arr_2[loop_index]);
        loop_index += 1;
    }

    // -- while loops
    while loop_index < arr_2.len() {
        println!("Arr : {}", arr_2[loop_index]);
        loop_index += 1;
    }

    // -- for loops
    for val in arr_2.iter() {
        println!("VAL : {}", val);
    }
}

fn tuple() {
    let my_tuple: (u8, String, f64) = (21, "Ricky".to_string(), 50_000.00);
    println!("Name : {}", my_tuple.1);

    let (v1, v2, v3) = my_tuple;
    println!("Age : {}", v1);

    //--

    let x: (i32, f64, u8) = (500, 6.4, 1);

    let five_hundred = x.0;
    let six_point_four = x.1;
    let one = x.2;

    //--

    let tup = (4330, 2.9, 4);
    let (x, y, z) = tup;
    println!("The value of y is: {}", y);
}

fn string() {
    let mut str1 = String::new();
    str1.push('A');
    str1.push_str("McDonald");

    for word in str1.split_whitespace() {
        println!("{}", word)
    }

    let str2 = str1.replace("A", "Another");
    println!("{}", str2);

    // random different character
    let str3 = String::from("x t r n b a z f v b b b ");
    let mut v1: Vec<char> = str3.chars().collect();
    v1.sort();
    v1.dedup();
    for char in v1 {
        println!("{}", char)
    }

    let str4: &str = "Random String";
    let mut str5: String = str4.to_string();
    println!("{}", str5);

    let byte_arr1 = str5.as_bytes();
    let str6 = &str5[0..6];
    println!("String Length, {}", str6.len());
    str5.clear();

    // Combine
    let str6 = String::from("Just Some");
    let str7 = String::from(" Words");
    let str8 = str6 + &str7;
    for char in str8.bytes() {
        println!("{}", char)
    }
}

fn casting() {
    let int_u8: u8 = 5;
    let int2_u8: u8 = 10;
    // using "as" >> you casting them
    let int3_u32: u32 = (int_u8 as u32) + (int2_u8 as u32);
    println!("{}", int3_u32)
}

fn enums() {
    enum Day {
        Monday,
        Tuesday,
        Wednesday,
        Thursday,
        Friday,
        Saturday,
        Sunday,
    }

    // function to this enumarated type
    impl Day {
        fn is_weekend(&self) -> bool {
            match self {
                Day::Saturday | Day::Sunday => true,
                _ => false,
            }
        }
    }

    let today: Day = Day::Monday;

    match today {
        Day::Monday => println!("Everyone hates monday"),
        Day::Tuesday => println!("Donut Day"),
        Day::Wednesday => println!("Hump day"),
        Day::Thursday => println!("Pay Day"),
        Day::Friday => println!("Almost weekend"),
        Day::Saturday => println!("Weekend"),
        Day::Sunday => println!("Weekend"),
    }

    println!("is today the weekend {}", today.is_weekend());
}

fn vector() {
    // Vector are Like Arrays
    let vec1: Vec<i32> = Vec::new();
    let mut vec2 = vec![1, 2, 3, 4];
    vec2.push(5);
    println!("1st : {}", vec2[0]);

    let second: &i32 = &vec2[1];
    match vec2.get(1) {
        Some(second) => println!("2nd : {}", second),
        None => println!("No 2nd Values"),
    }

    // Cycle & change values
    for index in &mut vec2 {
        *index *= 2
    }

    for index in &vec2 {
        println!("{}", index)
    }

    println!("Vector length {}", vec2.len());
    println!("Pop : {:?}", vec2.pop());
}

// ----- functions example

fn get_sum_2(x: i32, y: i32) -> i32 {
    // expression thats returned
    x + y
}
fn get_2(x: i32) -> (i32, i32) {
    return (x + 1, x + 2);
}
fn sum_list(list: &[i32]) -> i32 {
    let mut sum = 0;
    //  pinjam untuk val2 yg ada di list lalu di iter() dengan sum ditambah tiap val2 tersbut ke kosong
    for &val in list.iter() {
        sum += &val;
    }
    sum
}
fn functions() {
    println!("{}", get_sum_2(1, 90));

    // function return 1 value from 2 variable
    let (value_1, value_2) = get_2(85);
    println!("Nums : {} {}", value_1, value_2);

    // Vector in Function
    let num_list = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];
    println!("Sum of list = {} ", sum_list(&num_list));
}

// -----  to work for different data types = Generic

fn get_sum_gen<T: Add<Output = T>>(x: T, y: T) -> T {
    return x + y;
}
fn generics() {
    println!("5 + 1 = {}", get_sum_gen(5, 1));
    println!("5.7 + 12 = {}", get_sum_gen(5.7, 12.9));
}

// ----- Stack & Heap, Dealocation memory = Ownership

fn print_str(x: String) {
    println!("Strings {}", x)
}
fn print_return_str(x: String) -> String {
    println!("A String {}", x);
    x
}
fn change_string(name: &mut String) {
    name.push_str("is happy");
    println!("Message : {}", name)
}
fn ownership() {
    let str1 = String::from("World");
    let str2 = str1.clone();
    // print_str(str1);
    let str3 = print_return_str(str1);
    println!("str3  : {}", str3)
}

// ----- to store key-value pairs = Hash Map

fn hashmap() {
    let mut heroes = HashMap::new();
    heroes.insert("Superman", "Clark Kent");
    heroes.insert("Batman", "Bruce Wayne");
    heroes.insert("Flash", "Barry Allen");

    for (k, v) in heroes.iter() {
        println!("{} = {}", k, v)
    }

    println!("length {}", heroes.len());

    // check spesific keys in hashmap
    if heroes.contains_key(&"Batman") {
        let the_batman = heroes.get(&"Batman");
        match the_batman {
            Some(x) => println!("Batman is a hero"),
            None => println!("Batman is not a hero"),
        }
    }
}

// ----- to custom data type = Struct

fn structs() {
    struct Customer {
        name: String,
        address: String,
        balance: f32,
    }

    let mut bob = Customer {
        name: String::from("Bob Smith"),
        address: String::from("Pepaya St"),
        balance: 290.12,
    };

    bob.address = String::from("Ayam St");
    println!("{}", bob.address)
}

// -----  can be used in any struct, like OOP constructor & interface = Traits

fn traits() {
    const PI: f32 = 3.141592;

    trait Shape {
        fn new(length: f32, width: f32) -> Self;
        fn area(&self) -> f32;
    }

    struct Rectangle {
        length: f32,
        width: f32,
    }
    struct Circle {
        length: f32,
        width: f32,
    }

    impl Shape for Rectangle {
        fn new(length: f32, width: f32) -> Rectangle {
            return Rectangle { length, width };
        }
        fn area(&self) -> f32 {
            return self.length * self.width;
        }
    }
    impl Shape for Circle {
        fn new(length: f32, width: f32) -> Circle {
            return Circle { length, width };
        }
        fn area(&self) -> f32 {
            return (self.length / 2.0).powf(2.0) * PI;
        }
    }

    let rec: Rectangle = Shape::new(10.0, 12.0);
    let circle: Circle = Shape::new(12.4, 16.8);

    println!(
        "rectangle area : {}, circle area : {} ",
        rec.area(),
        circle.area()
    )
}
