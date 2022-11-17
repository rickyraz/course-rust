pub fn iter() {
    let mut arr = [1, 2, 3, 4, 5];
    for val in arr.iter() {
        println!("{}", val);
    }

    let mut iter1 = arr.iter();
    println!("1st {:?}", iter1.next())
}
