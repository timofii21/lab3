#[test]
fn main() {
    let s1: &str = "hello";
    let s: String = format!("{}, {}", s1, "world!");
    println!("{s}");
    assert_eq!(s, "hello, world!");
}