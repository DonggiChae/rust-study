// fn main() {
//     let s = "안녕하세요";
//     println!("{}",s[0]); // 직접 n번째 문자를 가져올 수 없다.
// }


// fn main() {
//     let s = "안녕하세요";
//     // 첫 1문자를 가져온다.
//     let ch = s.chars().nth(0).unwrap();
//     println!("{}", ch); // 안
//     // 3번째 문자를 가져온다.
//     let ch = s.chars().nth(2).unwrap();
//     println!("{}", ch); // 하
// }


fn main() {
    // 첫 번째 문자를 출력 
    // 영어는 1바이트
    let s2 = "abcdefg";
    println!("{}", &s2[0..1]); // a

    let s = "안녕하세요";

    // 첫 번째 1문자를 출력 
    // 한글은 3바이트
    let ch = &s[..3];
    println!("{}", ch); // 안
    
    // 세 번째 1문자를 출력 
    let ch = &s[6..9];
    println!("{}", ch); // 하
}