use rand::Rng;

// 전체 미로의 크기 지정
// const 상수명: 타입 = 값;
const MAP_N: usize = 25;

fn main() {
    // 난수 생성기 준비
    let mut rng = rand::thread_rng();
    // 미로 초기화 
    // let mut 변수명 = [[초깃값; 배열길이]; 배열길이];
    let mut maze = [[0; MAP_N]; MAP_N]; // 배열 변수 선언
    // 둘레를 벽으로 감싸기
    for n in 0..MAP_N {
        maze[n][0] = 1; // 상
        maze[0][n] = 1; // 좌
        maze[n][MAP_N-1] = 1; // 우
        maze[MAP_N-1][n] = 1; // 하
    }
    // 2칸마다 1개의 벽을 설치 
    for y in 2..MAP_N-2 {
        for x in 2..MAP_N-2 {
            if x % 2 == 1 || y % 2 == 1 { continue; }
            maze[y][x] = 1; // 벽
            // 상하좌우 중 어느 하나를 벽으로 만들기
            let r = rng.gen_range(0..=3);
            //match 문 
            match r {
                0 => maze[y-1][x] = 1, // 상
                1 => maze[y+1][x] = 1, // 하
                2 => maze[y][x-1] = 1, // 좌
                3 => maze[y][x+1] = 1, // 우
                _ => {},
            }
        }
    }
    // 미로 출력 
    let tiles = ["  ", "##"]; // 0과 1을 각각 흰색과 검은색 타일로 치환
    for y in 0..MAP_N {
        for x in 0..MAP_N {
            print!("{}", tiles[maze[y][x]]);
        }
        println!("");
    }
}
