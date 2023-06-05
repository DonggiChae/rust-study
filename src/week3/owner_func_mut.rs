// 인수의 문자열을 변경하는 함수
// &mut 가변 참조자
fn add_quote(msg: &mut String) {
    msg.insert(0, '"');
    msg.push('"');
}

fn main() {
    let mut msg = String::from("Beautiful");
    println!("{}", msg); 
    add_quote(&mut msg); 
    println!("{}", msg); 
}