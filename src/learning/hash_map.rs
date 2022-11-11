fn hashMapExample() {
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
