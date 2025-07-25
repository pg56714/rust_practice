// // enum IpAddrKind {
// //     V4(u8, u8, u8, u8),
// //     V6(String),
// // }

// struct Ipv4Addr {
//     // --省略--
// }

// struct Ipv6Addr {
//     // --省略--
// }
// enum IpAddr {
//     V4(Ipv4Addr),
//     V6(Ipv6Addr),
// }

// fn main() {
//     // let four = IpAddrKind::V4;
//     // let six = IpAddrKind::V6;

//     // route(IpAddrKind::V4);
//     // route(IpAddrKind::V6);

//     // let home = IpAddr {
//     //     kind: IpAddrKind::V4,
//     //     address: String::from("127.0.0.1"),
//     // };
//     // let home = IpAddr::V4(String::from("127.0.0.1"));
//     let home = IpAddr::V4(127, 0, 0, 1);

//     // let loopback = IpAddr {
//     //     kind: IpAddrKind::V6,
//     //     address: String::from("::1"),
//     // };
//     let loopback = IpAddr::V6(String::from("::1"));

// }

// // fn route(ip_kind: IpAddrKind) {}
// enum Message {
//     Quit,
//     Move { x: i32, y: i32 },
//     Write(String),
//     ChangeColor(i32, i32, i32),
// }

// fn main() {}

// 同樣的 enum 可以用不同的方式來定義結構體和元組結構體
// struct QuitMessage; // 類單元結構體
// struct MoveMessage {
//     x: i32,
//     y: i32,
// }
// struct WriteMessage(String); // 元組結構體
// struct ChangeColorMessage(i32, i32, i32); // 元組結構體

// fn main() {}

// fn main() {
//     enum Message {
//         Quit,
//         Move { x: i32, y: i32 },
//         Write(String),
//         ChangeColor(i32, i32, i32),
//     }

//     impl Message {
//         fn call(&self) {
//             // 在此定義方法本體
//         }
//     }

//     let m = Message::Write(String::from("hello"));
//     m.call();
// }

// enum Option<T> {
//     None,
//     Some(T),
// }

// fn main() {
//     let some_number = Some(5);
//     let some_char = Some('e');

//     let absent_number: Option<i32> = None;
// }

// fn main() {
//     let x: i8 = 5;
//     let y: Option<i8> = Some(5);

//     let sum = x + y;
// }

// #[derive(Debug)] // 這讓我們可以顯示每個州
// enum UsState {
//     // Alabama,
//     Alaska,
//     // --省略--
// }

// enum Coin {
//     Penny,
//     // Nickel,
//     // Dime,
//     Quarter(UsState),
// }

// fn value_in_cents(coin: Coin) -> u8 {
//     match coin {
//         Coin::Penny => {
//             println!("幸運幣！");
//             1
//         }
//         // Coin::Nickel => 5,
//         // Coin::Dime => 10,
//         Coin::Quarter(state) => {
//             println!("此 25 美分所屬的州為 {:?}!", state);
//             25
//         }
//     }
// }
// fn main() {
//     value_in_cents(Coin::Quarter(UsState::Alaska));
//     value_in_cents(Coin::Penny);
// }

// fn main() {
//     fn plus_one(x: Option<i32>) -> Option<i32> {
//         match x {
//             None => None,
//             Some(i) => Some(i + 1),
//         }
//     }

//     let five = Some(5);
//     let six = plus_one(five);
//     let none = plus_one(None);
//     println!("{:?}, {:?}, {:?}", six, none, plus_one(Some(10)));
// }

// fn main() {
//     let dice_roll = 9;
//     match dice_roll {
//         3 => add_fancy_hat(),
//         7 => remove_fancy_hat(),
//         other => move_player(other),
//     }

//     fn add_fancy_hat() {}
//     fn remove_fancy_hat() {}
//     fn move_player(num_spaces: u8) {}
// }

// fn main() {
//     let dice_roll = 9;
//     match dice_roll {
//         3 => add_fancy_hat(),
//         7 => remove_fancy_hat(),
//         _ => reroll(),
//     }

//     fn add_fancy_hat() {}
//     fn remove_fancy_hat() {}
//     fn reroll() {}
// }

// fn main() {
//     let dice_roll = 9;
//     match dice_roll {
//         3 => add_fancy_hat(),
//         7 => remove_fancy_hat(),
//         _ => (),
//     }

//     fn add_fancy_hat() {}
//     fn remove_fancy_hat() {}
// }

// fn main() {
//     let config_max = Some(3u8);
//     match config_max {
//         Some(max) => println!("最大值被設為 {}", max),
//         _ => (),
//     }
// }

fn main() {
    let config_max = Some(3u8);
    if let Some(max) = config_max {
        println!("最大值被設為 {}", max);
    }
}
