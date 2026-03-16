#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
    // --이하 생략--
}
enum Coin {
    Panny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Panny => {
            println!("Lucky panny!");
            1
        }
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quater from {:?}!", state);
            25
        }
    }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    // match는 가능한 경우를 다 다뤄야 함
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}

fn main() {
    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);

    println!("{} {}", six.unwrap(), none.unwrap_or(0));

    let dice_roll = 9;
    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        other => move_player(other), // 포괄 패턴, other이라는 변수에 값을 할당 후 함수에 넘겨줌
        // _ => reroll(), // 값이 따로 필요 없을때는 _를 이용해 포괄패턴 사용 가능 (like default in C, C++)
    }

    // if let 문법으로 간단하게 만들기
    // let config_max = Some(3u8);
    // match config_max {
    //     Some(max) => println!("The maximum is configured to be {}", max),
    //     _ => (), // 아무것도 하기 싫을땐 명시적으로 표현
    // }

    // but match가 강제했던 철저한 검사를 안함
    let config_max = Some(3u8);
    if let Some(max) = config_max {
        println!("The maximum is configured to be {}", max);
    } 
    else { // else를 이용해 기본 케이스를 표현가능
        println!("This is else");
    }
}

fn add_fancy_hat() {}
fn remove_fancy_hat() {}
fn move_player(num_spaces: u8) {}
fn reroll() {}