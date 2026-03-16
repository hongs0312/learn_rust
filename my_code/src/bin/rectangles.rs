//디버깅 정보 출력을 위한 명시적 동의
#[derive(Debug)] //외부 속성(outer attribute)
struct Ractangle {
    width: u32,
    height: u32,
}

impl Ractangle {
    // 연관함수(associated function): 이 블럭 안에 구현된 함수들

    // 메서드의 첫번째 매개변수는 자기자신(self)
    // &self == self: &self의 축약형
    // 만약 값을 변경하고 싶으면 &self 대신 &mut self 사용
    fn area(&self) -> u32 {
        self.width * self.height
    }

    // 필드명과 똑같은 이름의 메서드 만들기 가능
    // 보통 getter를 구현할때 사용
    fn width(&self) -> bool {
        self.width > 0
    }

    fn can_hold(&self, other: &Ractangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}
// impl블럭은 여러개 생성 가능
impl Ractangle {
    // 인자로 self를 받지 않음 -> 메서드 아님
    // 생성자로 자주 이용됨
    // 호출시 .이 아닌 :: 이용
    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }
}

fn main() {
    let rectangle = Ractangle {
        width: 30,
        height: 50,
    };

    println!("rectangle is {:#?}", rectangle);

    println!(
        "The area of the ractangle is {} square pixels",
        rectangle.area()
    );
    println!("{}", rectangle.width());

    // ++ 러스트에선 자동참조가 되므로 참조자를 명시적으로 적을필요 없음
    // p1.distance(&p2); <-- 더 깔끔
    // (&p1).distance(&p2);
    // 이 둘은 서로 같은 표현

    let scale = 2;
    let rect1 = Ractangle {
        width: dbg!(30 * scale),
        height: 50,
    };
    //dbg!: stdout이 아니라 stderr로 정보 출력
    dbg!(&rect1);

    let rect2 = Ractangle {
        width: 10,
        height: 40,
    };
    let rect3 = Ractangle {
        width: 60,
        height: 45,
    };

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));

    let sqare = Ractangle::square(10);
    println!("{:#?}", sqare);
}