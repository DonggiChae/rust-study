fn main() {
    let nums = vec![1, 2, 3];
    println!("{:?}", nums);
}


// vec! 매크로를 사용하지 않으면 밑에처럼 사용해야한다.
// fn main() {
//     let mut nums = Vec::new();
//     nums.push(1);
//     nums.push(2);
//     nums.push(3);
//     println!("{:?}", nums);
// }