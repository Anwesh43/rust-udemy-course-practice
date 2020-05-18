fn use_slice(slice : &[i32]) {
    println!("slice length {} and first element {}", slice.len(), slice[0]);
}
fn main() {
    let a : [i32; 5] = [1, 2, 3, 4, 5];
    use_slice(&a[1..4]);
    use_slice(&mut a);
}
