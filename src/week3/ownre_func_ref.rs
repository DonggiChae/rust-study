// &을 붙인다는 것은 함수의 인수를 '참조자로 만드는 것이고, '빌림'이라고 한다.

fn main() {
    let g1 = String::from("beautiful");
    show_message(&g1); // 참조 값을 전달 
    println!("{}", g1); // 소유권은 이동하지 않음 
}

fn show_message(message: &String) { // &
}

// &String과 &str은 거의 동일하게 사용 되지만, &str은 슬라이스이다.

