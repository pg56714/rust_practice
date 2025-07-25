// #[cfg(test)]
// mod tests {
//     #[test]
//     fn it_works() {
//         let result = 2 + 2;
//         assert_eq!(result, 4);
//     }
// }

// #[cfg(test)]
// mod tests {
//     #[test]
//     fn exploration() {
//         assert_eq!(2 + 2, 4);
//     }
// }

// /// 回傳數字 5
// ///
// /// # 範例
// /// ```
// /// let x = chapter11::five();
// /// assert_eq!(x, 5);
// /// ```
// pub fn five() -> i32 {
//     5
// }

// #[derive(Debug)]
// struct Rectangle {
//     width: u32,
//     height: u32,
// }

// impl Rectangle {
//     fn can_hold(&self, other: &Rectangle) -> bool {
//         self.width > other.width && self.height > other.height
//     }
// }

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn larger_can_hold_smaller() {
//         let larger = Rectangle {
//             width: 8,
//             height: 7,
//         };
//         let smaller = Rectangle {
//             width: 5,
//             height: 1,
//         };

//         assert!(larger.can_hold(&smaller));
//     }
// }

// pub fn greeting(_name: &str) -> String {
//     // format!("哈囉{}!", name)
//     format!("哈囉")
// }

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn greeting_contains_name() {
//         let result = greeting("卡爾");
//         // assert!(result.contains("卡爾"));
//         assert!(
//             result.contains("卡爾"),
//             "打招呼時並沒有喊出名稱，其數值為 `{}`",
//             result
//         );
//     }
// }

// #![allow(unused)]
// pub struct Guess {
//     value: i32,
// }

// impl Guess {
//     pub fn new(value: i32) -> Guess {
//         if value < 1 {
//             panic!(
//                 "猜測數字必須大於等於 1，取得的數值是 {}。",
//                 value
//             );
//         } else if value > 100 {
//             panic!(
//                 "猜測數字必須小於等於 100，取得的數值是 {}。",
//                 value
//             );
//         }

//         Guess { value }
//     }
// }

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     #[should_panic(expected = "小於等於 100")]
//     fn greater_than_100() {
//         Guess::new(200);
//     }
// }

// #[cfg(test)]
// mod tests {
//     #[test]
//     fn it_works() -> Result<(), String> {
//         if 2 + 2 == 4 {
//             Ok(())
//         } else {
//             Err(String::from("二加二不等於四"))
//         }
//     }
// }

// pub fn add_two(a: i32) -> i32 {
//     internal_adder(a, 2)
// }

// fn internal_adder(a: i32, b: i32) -> i32 {
//     a + b
// }

// #[cfg(test)]
// mod tests {
//     use super::*; // 引入上層模組的函式

//     #[test]
//     fn internal() {
//         assert_eq!(4, internal_adder(2, 2));
//     }
// }

// 單元測試通常放在同一個檔案中，而整合測試則放在 `tests` 資料夾中。
// src/lib.rs
pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(add(2, 3), 5);
    }
}
