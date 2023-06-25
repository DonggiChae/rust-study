// 다음 코드는 에러가 발생하나요?

#[derive(Clone)]
struct Person {
    name: String,
    age: i32,
}
impl Person {
    fn new(name: &str, age: i32) -> Self {
        Self { name: name.to_string(), age }
    }
}

fn main() {
    let alex = Person::new("Alex", 18);
    let mut betty = alex; // <-- 소유권이 이동 .clone()
    betty.name = String::from("Betty");
    println!("{},{}", alex.name, alex.age);  // <--에러 발생
    println!("{},{}", betty.name, betty.age);
}
