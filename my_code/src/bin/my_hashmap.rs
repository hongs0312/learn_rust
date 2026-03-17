use std::collections::HashMap;
fn main() {
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    let team_name = String::from("Blue");
    let score = scores.get(&team_name).copied().unwrap();

    // 반복문을 이용해 순회 가능
    for (key, value) in &scores {
        println!("{key}, {value}");
    }

    // let field_name = String::from("Favorite color");
    // let field_value = String::from("Blue");

    // let mut map = HashMap::new();
    // map.insert(field_name, field_value); // 이 순간부터 field_name, field_value의 소유권은 소멸됨

    // 해시맵 값 덮어쓰기
    scores.insert(String::from("Blue"), 25);
    println!("{:?}", scores);

    // entry 함수를 이용해 해당 키값이 있는지 없는지 검사 가능
    // entry 함수의 반환값 == Entry 열거형
    scores.entry(String::from("Yellow")).or_insert(50);
    scores.entry(String::from("Red")).or_insert(100);

    println!("{:?}", scores);

    let text = "hello world wonderful world";

    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        // entry로 해당 키가 있는지 검사 후 없으면 0 삽입,
        // 해당 참조자 반환
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }
    println!("{:?}", map);
}