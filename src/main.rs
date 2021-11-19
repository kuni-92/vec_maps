use std::collections::HashMap;

fn main() {
    let mut v: Vec<i32> = Vec::new();
    v.push(1);
    v.push(2);
    v.push(3);

    for n in &v {
        println!("Vec number: {}", n);
    }

    let none = v.get(100);
    println!("none value: {:?}", none);

    let key_text: Vec<String> = vec![String::from("one"), String::from("two"), String::from("three")];

    let mut map = HashMap::new();

    for k in key_text {
        map.insert(k, 10);
    }

    for (key, value) in map {
        println!("map value key: {}, value: {}", key, value);
    }
}
