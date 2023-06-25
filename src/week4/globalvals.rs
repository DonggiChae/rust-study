// 전역 가변 변수 선언 여기에서는 unsafe를 선언하지 않아도 됨
static mut TAX: f32 = 0.1;

// 가변 전역변수를 사용하는 거 자체가 문제가 된다.
fn main() {
    // 안전하지 않다는 것을 명시 
    unsafe {
        // 가변 전역 변수 사용 
        println!("Price: {}", TAX * 300.0);
        // 가변 전역 변수의 값 변경
        TAX = 0.08;
        println!("Price: {}", TAX * 300.0);
    }
}