use std::collections::HashSet;

fn main() {
    let mut hs = HashSet::new();
    hs.insert("alpha");
    hs.insert("gamma");
    hs.insert("beta");
    println!("{:?}", hs);
    let alpha_inserted =  hs.insert("alpha");
    println!("{:?}", hs);
    if alpha_inserted {
        println!("alpha is inserted");
    } else {
        println!("alpha is not inserted");
    }
    if hs.contains("alpha") {
        println!("hs has alpha");
    }
    if hs.remove("alpha") {
        println!("alpha is removed");
    }
    println!("{:?}", hs);
    let _1_to_5 : HashSet<_> = (1..=5).collect();
    let _6_to_10 : HashSet<_> = (6..=10).collect();
    let _1_to_10 : HashSet<_> = (1..=10).collect();
    let _2_to_8 : HashSet<_> = (2..=8).collect();
    println!("is {:?} disjointed {:?} {}", _1_to_5, _6_to_10, _1_to_5.is_disjoint(&_6_to_10));
    println!("is {:?} subset of {:?} {}", _2_to_8, _1_to_10, _2_to_8.is_subset(&_1_to_10));
    println!("union of {:?} and {:?} is {:?}", _1_to_5, _6_to_10, _1_to_5.union(&_6_to_10));
    println!("intersection of {:?} and {:?} is {:?}", _1_to_5, _2_to_8, _1_to_5.intersection(&_2_to_8));
    println!("intersection of {:?} and {:?} is {:?}", _1_to_5, _6_to_10, _1_to_5.intersection(&_6_to_10));
    println!("difference of {:?} and {:?} is {:?}", _1_to_5, _2_to_8, _1_to_5.difference(&_2_to_8));
    println!("symetric difference of {:?} and {:?} is {:?}", _1_to_5, _2_to_8, _1_to_5.symmetric_difference(&_2_to_8));
}
