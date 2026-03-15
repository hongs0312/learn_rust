// 열거형: 어떤값이 가능한 여러개의 값 중 하나라는걸 표현

// 따로 설정하는것 보다 배리언트에 직접 정의 가능
// enum IpAddrKind {
//     // 각각이 배리언트
//     V4,
//     V6,
// }
// struct IpAddr {
//     kind:IpAddrKind,
//     address: String,
// }

enum IpAddr {
    V4(u8, u8, u8, u8),
    V6(String),
}

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

// 아래 4개의 구조체 정의를 하나로 묶을 수 있음
// struct QuitMessage;
// struct MoveMessage {
//     x: i32,
//     y: i32,
// }
// struct WriteMessage(String);
// struct ChangeColorMessage(i32, i32, i32);

impl Message {
    fn call(&self) {

    }
}

// fn route(ip_kind: IpAddrKind) {}

fn main() {
    // let four = IpAddrKind::V4;
    // let six = IpAddrKind::V6;

    // route(IpAddrKind::V4);
    // route(IpAddrKind::V6);

    // String을 입력 받아서 해당하는 자료형의 인스턴스 결과를 만드는 함수
    // 열거형을 정의하면 생성자가 자동으로 정의
    let home = IpAddr::V4(127, 0, 0, 1);
    let loopback = IpAddr::V6(String::from("::1"));

    let m = Message::Write(String::from("hello"));
    m.call();
}