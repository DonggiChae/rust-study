// fn main() {
//     let s = "die".to_string();
//     echo(s); // ← 소유권이 이동한다
//     println!("{}", s);
// }
// // println!을 모방한 함수
// fn echo(s: String) {
//     println!("{}", s);
// }

// println! 소유권이 이동하지 않음
fn main() {
    let s = "die".to_string();
    println!("{}", s); // 소유권이 이동하지 않는다
    println!("{}", s); // 
}