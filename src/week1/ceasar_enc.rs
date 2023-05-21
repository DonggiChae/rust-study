fn encycpt(test: &str, shift:i16) -> String {
    let code_a = 'A' as i16;
    let code_z = 'Z' as i16;

    let mut result = String::new();

    for ch in text.chars() {
        let mut code = ch as i16;
        if code_a <= code && code <= code_z { //Rust 특징 && 쓰기
            code = (code - code_a + shift + 26) % 26 + code_a;
        }
    }
    result.push(code as u8 as char); // i16에서 char로 바로 변환할 수 없기 때문데 u8로 변환한 뒤 다시 char로 변환한다.
}

fn main () {
    let enc = encycpt("HELLO", 3);
    let dec = encycpt(&enc, -3);
    println!("{} -> {}", enc, dec);
}

