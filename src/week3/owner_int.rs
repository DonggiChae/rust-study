fn main() {
    let g1 = 30;
    let g2 = g1; // 값이 자동으로 복사됨
    println!("{}", g1); // ok
    println!("{}", g2); // ok
}

// 데이터 타입 기본 - 복사됨
// 기본형 외의 타입 - 이동
// 기본형 이외의 데이터 타입에서도 Copy 트레잇을 구현한 데이터 타입이라면 데이터를 보기사 할 수 있다.
