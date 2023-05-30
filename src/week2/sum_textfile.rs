// 같은 모듈의 서브 모듈일 경우 중괄호로 감싸 기입할 수 있다.
use std::{env, fs};

fn main() {
    let args = env::args();
    let mut total:f64 = 0.0;
    for (i, fname) in args.enumerate() {
        if i == 0 { continue; }
        // 텍스트 파일을 읽어들임 
        let text = fs::read_to_string(fname).unwrap();
        // 한 줄씩 분리 
        let lines = text.split('\n');
        for line in lines {
            let n:f64 = match line.parse() {
                Ok(v) => v,
                Err(_) => 0.0,
            };
            total += n;
        }
    }
    // 결과 표시 --- (*7)
    println!("{}", total);
}