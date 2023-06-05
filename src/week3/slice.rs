// fn main() {
//     let s = String::from("beep");
//     let ss = &s[0..3];
//     println!("{}", ss);
// }


fn sum_slice(items: &[i64]) -> i64 {
    let mut total = 0;
    for i in items {
        total += i;
    }
    total // 합계 반환
}

fn main() {
    // 배열 초기화
    let a = [1,2,3,4,5,6,7,8,9,10];
    println!("a={}", sum_slice(&a[..]));
    println!("{:?}", a);
    // a=55
    // 벡터 초기화
    let b = vec![1,2,3,4,5,6,7,8,9,10];
    println!("b={}", sum_slice(&b[..]));
    println!("{:?}", b);
    // b=55 
}