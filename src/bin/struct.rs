// 일반적인 구조체 정의
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

// 튜플 구조체 정의
// 일반 구조체처럼 구조체병 쓸 필요 없이 타입만 명시
struct Color (i32, i32, i32);
struct Point (i32, i32, i32);

//필드 초기화 축약법, 매개변수명이랑 필드명이 같으면 생략 가능
fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        username,
        email,
        sign_in_count: 1,
    }
}

fn print_user(user: &User) {
    println!("User info:");
    println!("       active: {}", user.active);
    println!("     username: {}", user.username);
    println!("        email: {}", user.email);
    println!("sign_in_count: {}", user.sign_in_count);
    println!();
}

fn main() {
    let user1 = build_user(
        String::from("someone@example.com"), 
        String::from("someusername123"));

    print_user(&user1);

    // 구조체 업데이트 문법을 사용하지 않았을때
    // let user2 = User {
    //     active: user1.active,
    //     username: user1.username,
    //     email: String::from("another@example.com"),
    //     sign_in_count: user1.sign_in_count,
    // };
    
    //구조체 업데이트 문법을 사용했을때
    let user2 = User {
        email: String::from("another@example.com"),
        ..user1 //무조건 끝에 적어야 함
    };
    // 구조체 업데이트 이후에는 user1 사용 불가
    // username 필드의 String이 user1에서 user2로 이동했기 때문
    // print_user(&user1);

    print_user(&user2);

    //둘다 i32 3개로 이루어진 구조체이지만 서로 다른 타입임
    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);

    println!("{} {} {}", black.0, black.1, black.2);
    println!("{} {} {}", origin.0, origin.1, origin.2);
}