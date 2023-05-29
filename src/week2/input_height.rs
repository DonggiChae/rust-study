fn main() {
    let mut height;

    loop {
        println!("키(cm) : ");
        height = input_f(0.0); 
        if height > 0.0 { break; }
        println!("숫자만 입력 가능합니다.");
    }

    let weight = 22.0 * (height / 100.0).powf(2.0);
    println!("표준 체중은 {:.1}kg 입니다.", weight);
}

// 표준 입력에서 문자열을 얻기 
fn input_str() -> String {
    let mut s = String::new();
    std::io::stdin()
        .read_line(&mut s)
        .expect("입력 에러");
    s.trim_end().to_string()
}
// error 처리시 match 구문 사용 추천
// 실제 서비스에서 panic이 발생하는 것을 방지하기 위해
// 표준 입력에서 실수를 얻기(실패시 def 반환)
fn input_f(def: f64) -> f64 {
    let s = input_str();
    // 공백 제거 후 숫자 값으로 변환
    match s.trim().parse() {
        Ok(v) => v,
        Err(_) => def,
    }
}