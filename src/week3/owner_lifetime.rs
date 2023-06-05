// // 메시지를 생성한 뒤 그 참조자를 반환하는 함수
// fn gen_message() -> &str {
//     let msg = String::from("beautiful");
//     return &msg;
// }

// fn main() {
//     let m = gen_message();
//     println!("{}", m);
// }
// 위에서는 소유자가 유효한 범위를 벗어나기 떄문에 값이 파기된다.

// 메시지 생성 함수
fn gen_message() -> String {
    let msg = String::from("beautiful");
    return msg; // 소유권이 함수의 반환 값으로 이동 
}
fn main() {
    let m = gen_message(); // 소유권은 m으로 이동 
    println!("{}", m); // ok
}