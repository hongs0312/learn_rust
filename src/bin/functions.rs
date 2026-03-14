fn main() {
    another_function(5);

    //표현식과 구문
    //구문 : 값을 반환하지 않음
    //표현식 : 값을 반환하며 평가
    //a = b = 5 같은 할당이 불가 (할당식은 구문이기 때문)

    //아래 전체코드 == 구문
    let y = {
        let x = 3;
        x + 1 //세미콜론 없음 == 표현식
    };
    //구문은 표현식을 포함할 수 있다

    println!("The value of y is: {y}");

    let x = five();

    println!("The value of x is {x}");
}
fn another_function(x: i32) {
    println!("The value of x is {x}");
}
fn five() -> i32 {
    5
}