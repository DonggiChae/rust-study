
use std::collections::HashMap;

// hashMap에 존재하지 않는 키를 취득하려고 할 때 에러 발생
// fn main() {
//     // HashMap을 생성해 초기화
//     let mut map = HashMap::new();
//     map.insert("A", 30);
//     map.insert("B", 50);
//     // 존재하지 않는 키를 취득
//     let d = map["D"];
//     println!("{}", d);
// }

fn main() {
    let mut map = HashMap::new();
    map.insert("A", 30);
    map.insert("B", 50);
    // 키가 존재하지 않는지 확인
    // Option 타입 사용
    match map.get("D") { 
        Some(v) => println!("D={}", v),
        None => println!("D는 존재하지 않음"),
    }
}