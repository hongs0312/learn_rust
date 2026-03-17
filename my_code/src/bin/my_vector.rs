fn main() {
    let mut v: Vec<i32> = Vec::new();

    // vec! 매크로를 이용해 벡터 선언가능
    // let v = vec![1, 2, 3];

    v.push(5);
    v.push(6);
    v.push(7);
    v.push(8);


    let v = vec![1, 2, 3, 4, 5];

    let third: &i32 = &v[2];
    println!("The third element is {third}");

    // get 메서드를 이용해 읽어오면 Option값으로 읽힘
    let third: Option<&i32> = v.get(2);
    match third {
        Some(third) => println!("The third element is {third}"),
        None => println!("There is no third element"),
    }

    // 아래와 같은 경우 불변 참조자가 유효해 6이 푸쉬되지 않고 패닉을 일으킴
    // 벡터는 충분한 공간이 없는경우 다른 넓은 공간으로 옮기고 값을 복사하기 때문에
    // 대여 규칙의 적용을 받음
    // let mut v = vec![1, 2, 3, 4, 5];

    // let first = &v[0];

    // v.push(6);

    let mut v = vec![100, 32, 57];

    for i in &v {
        println!("{i}");
    }

    // 아이템을 추가하지만 않는다면 대여규칙에 의해 안전함
    for i in &mut v {
        *i += 50;
    }

    // 열거형을 정의해서 다양한 타입을 담을 수도 있음
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let row = vec! [
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];


}