extern crate num_bigint; // 2018버젼부터는 생략 가능 

// BigInt 선언
use num_bigint::BigInt;

//BigInt  사용하지 않으면 오버플로우가 발생하는 곱셈시도 에러 발생
fn main() {
    // BingInt 오브젝트 생성
    let v = BigInt::from(1234);
    // 제곱 계산
    println!("{}", v.pow(5678));
}
