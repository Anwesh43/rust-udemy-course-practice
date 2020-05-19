use std::collections::HashMap;

fn main() {
    let mut hm : HashMap<String, i64> = HashMap::new();
    hm.insert("triangle".into(), 3);
    hm.insert("square".into(), 4);
    println!("{:?}", hm);
    for (key, value) in &hm {
        println!("{}:{}", key, value);
    }
    hm.insert("square".into(), 5);
    hm.entry("circle".into()).or_insert(1);
    hm.entry("hexagon".into()).or_insert(5);
    println!("{:?}", hm);
}
