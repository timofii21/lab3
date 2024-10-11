#[test]
fn main() {
    let s1: &str = "hello";
    let s: String = format!("{}, {}", s1, "world!");
    println!("{s}");
}

