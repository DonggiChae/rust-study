fn main() {
    let mut g1 = String::from("beautiful");
    g1 = show_message(g1); // 1
    println!("{}", g1); // ok 2 
}

// String을 받아 String을 반환하는 함수 3
fn show_message(message: String) -> String {
    println!("{}", message);
    return message;
}