// 다음 코드에서 에러가 발생한다?
// no 경고가 발생
// 사용하지 않은 인수가 있는경우 컴파일 할때 경고를 발생하는데 _(언더바)를 붙이면 해결할 수 있다.

struct TrapBox {
    damage: i32,
}
impl TreasureBox for TrapBox {
    fn open(&self, key: i32) -> bool {
        return true; // 어떤 키를 가지고 있어도 열림
    }
    fn check(&self) {
        println!("함정이었다. HP가 {} 감소했다.", self.damage);
    }
}