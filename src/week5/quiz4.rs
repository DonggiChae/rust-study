// 에러가 발생하나요?
// true
// AddAssign을 사용해야함
#[derive(Debug)]
struct Point<T> {
    x: T,
    y: T,
}

impl<T> Point<T> where T: std::ops::Add {
    fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
    fn add(&mut self, pt: Point<T>) {
        self.x += pt.x;
        self.y += pt.y;
    }
}
fn main() {
    let mut pt = Point::new(10, 10);
    println!("{:?}", pt);
    pt.add(Point{ x:20, y:30 });
    println!("{:?}", pt);
}