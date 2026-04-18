struct Point<T> {
    x: T,
    y: T,
}

impl<T> Point<T> { // impl뒤의 제네릭 변수 명은 같은 이름을 사용하는게 관례
    fn x(&self) -> &T {
        &self.x
    }
}

fn main() {
    let p = Point {x: 5, y: 10};

    println!("p.x = {}", p.x());
}