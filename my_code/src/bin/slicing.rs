//&str이 &String보다 더 일반적으로 사용 가능
//&String이랑 &str값 모두 인자로 사용 가능
fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    &s[..]
}

fn main () {
    // let s = String::from("hello world!");

    // let word = first_word(&s);

    //word가 불변참조자이므로 불변참조자가 있으면 가변참조자 못만듦
    //s.clear();

    // let my_string = String::from("hello world");

    //아래 용법은 모두 사용 가능
    // let word = first_word(&my_string[0..6]);
    // let word = first_word(&my_string[..]);

    // let word = first_word(&my_string);

    let my_string_literal = "hello world";

    // let word = first_word(&my_string_literal[0..6]);
    // let word = first_word(&my_string_literal[..]);

    let word = first_word(my_string_literal);

    println!("the first word is: {}", word);
}