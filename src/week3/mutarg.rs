// 인수의 값에 2를 곱해 반환하는 함수 
// *변수명 역참조
fn x2(arg: &mut i32) {
    *arg = *arg * 2;
}

fn main() {
    let mut v = 16;
    x2(&mut v); 
    println!("{}", v);
}