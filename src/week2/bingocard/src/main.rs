
use rand::seq::SliceRandom;

fn main() {

    let mut nums = [0; 75];
    for i in 1..=75 { nums[i-1] = i; }

    let mut rng = rand::thread_rng(); // 난수 생성기 오브젝트 생성
    nums.shuffle(&mut rng); // 섞기 , 위에 선언하였기 때문에 사용 가능
    

    for y in 0..5 {
        for x in 0..5 {
            let i = y * 5 + x; // 매번 새로운 값을 생성하기 때문에 불변으로 설정하지 않아도 문제가 없다.
            if i == 12 { 
                print!("  *,"); //와일드 카드, 파이썬과는 다르게 러스트는 배열의 데이터 타입을 일치시켜야 한다. 그래서 12일때 '*'을 출력하게 했다.
            } else {
                print!("{:3},", nums[i]);
            }
        }
        println!("");
    }
}
