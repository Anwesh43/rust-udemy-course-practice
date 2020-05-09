fn check_country(country_code : i64) {
    let country = match country_code {
        44 => "UK",
        60 => "USA",
        1..=1000 => "Any Country",
        _ => "invalid"
    };
    println!("country is {}", country);
}
fn main() {
    check_country(44);
    check_country(60);
    check_country(78);
    check_country(999);
    check_country(1001);
}
