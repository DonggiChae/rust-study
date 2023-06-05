// fn main() {
//     {
//         let s1 = String::from("no Limit");
//         println!("{}", s1);
//     }
//     // 블록을 벗어나면 s1은 파기
// }

fn main() {
    // 블록1
    {
        let s1 = String::from("just do it");
        let s3 = String::from("the only way is up");
        // 블록 2
        {
            let s2 = s1;
            println!("{}", s2);
        }
        // s2의 값은 여기서 파기
        println!("{}", s3);
    }
    // s3의 값은 여기서 파기
}