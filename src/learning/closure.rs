fn closures_in_fn() {
    fn use_func<T>(a: i32, b: i32, func: T) -> i32
    where
        T: Fn(i32, i32) -> i32,
    {
        func(a, b)
    }

    let sum = |a: i32, b: i32| a + b;
    let prod = |a: i32, b: i32| a * b;
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
