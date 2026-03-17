fn main() {
    let mut s = String::new();

    // to_string을 이용해 문자열 리터럴을 String으로 바꿀 수 있음
    let data = "initial contents";
    let s = data.to_string();

    // 문자열 리터럴에서도 잘 작동함
    let s = "initial contents".to_string();

    // form을 이용해 초기화도 가능
    let s = String::from("initial contents");

    //push str을 이용해 문자열 합치기
    let mut s = String::from("foo");
    s.push_str("bar");

    // s1은 그대로, s2는 참조자로 호출하는 이유
    // 문자열 끼리 더할때 호출되는 add 함수의 인자에 맞춰야하기 때문
    // fn add(self, s: &str) -> String {}이기 때문이다
    // self로 수유권을 가져가기 때문에 앞에 있는 문자열은 이후 사용 불가
    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s3 = s1 + &s2; // s1은 이동 돼서 더이상 사용 불가, s2는 사용 가능

    // format! 매크로를 활용해 문자열 한 번에 합치기
    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    // format! 매크로는 각 문자열의 참조자를 이용하므로 소유권을 가져가지 않음
    let s = format!("{s1}-{s2}-{s3}");

    // // 문자열에 인덱싱 문법을 사용하려는 경우 에러 발생
    // let s1 = String::from("hello");
    // let h = s1[0];

    // 문자열 길이가 5처럼 보이지만 utf-8 기준 한글은 한 글자당 3바이트 이므로 길이는 15이다
    // 따라서 인덱싱시 O(1)에 실행되길 바라지만 문자마다 바이트수가 다른 등 의 이유로 인해
    // 현실적으로 불가능하므로 러스트는 문자열에서 인덱싱 문법을 지원하지 않음
    let hello = String::from("안녕하세요");

    // 러스트의 관점에서 문자열은 바이트, 스칼라 값, 문자소 클러스터(문자와 가장 가까움)
    // 3가지의 방식으로 문자열을 해석할 수 있음
    // 따라서 인덱싱, 슬라이싱은 문제가 생김

    // 문자열 반복을 위해선 명시적으로 chars()와 같이 메서드를 명시하는 방법이 있음
    for c in "안녕".chars() {
        println!("{c}");
    }

    for b in "안녕".bytes() {
        println!("{b}");
    }
}