// fn main() {
//     {
//         // s 在此處無效，因為它還沒宣告
//         let s = "hello";   // s 在此開始視為有效
//         println!("s 的值為: {s}"); // 使用 s
//     }   // 此作用域結束， s 不再有效
// }

// #![allow(unused)]
// fn main() {
//     let mut s = String::from("hello");
//     s.push_str(", world!"); // push_str() 將字面值加到字串後面

//     println!("{}", s); // 這會印出 `hello, world!`
// }

// #![allow(unused)]
// fn main() {
//     // x 取得數值 5，然後拷貝（copy）了一份 x 的值給 y。
//     // let x = 5;
//     // let y = x;

//     // s1 取得字串 "hello"，然後將 s1 的所有權轉移給 s2。
//     // 這樣 s1 就不再有效了。
//     let s1 = String::from("hello");
//     // let s2 = s1;
//     // println!("{}, world!", s1);

//     let s2 = s1.clone(); // 使用 clone() 來複製 s1 的內容到 s2
//     println!("s1 = {}, s2 = {}", s1, s2);
// }

// fn main() {
//     let s = String::from("hello");  // s 進入作用域

//     takes_ownership(s);        // s 的值進入函式
//                                            // 所以 s 也在此無效

//     let x = 5;                        // x 進入作用域

//     makes_copy(x);           // x 本該移動進函式裡
//                                           // 但 i32 有 Copy，所以 x 可繼續使用

// } // x 在此離開作用域，接著是 s。但因為 s 的值已經被移動了
//   // 它不會有任何動作

// fn takes_ownership(some_string: String) { // some_string 進入作用域
//     println!("{}", some_string);
// } // some_string 在此離開作用域並呼叫 `drop`
//   // 佔用的記憶體被釋放

// fn makes_copy(some_integer: i32) { // some_integer 進入作用域
//     println!("{}", some_integer);
// } // some_integer 在此離開作用域，沒有任何動作發生

// #![allow(unused)]
// fn main() {
//     let s1 = gives_ownership();         // gives_ownership 移動它的回傳值給 s1

//     let s2 = String::from("哈囉");     // s2 進入作用域

//     let s3 = takes_and_gives_back(s2);  // s2 移入 takes_and_gives_back
//                                         // 該函式又將其回傳值移到 s3
// } // s3 在此離開作用域並釋放
//   // s2 已被移走，所以沒有任何動作發生
//   // s1 離開作用域並釋放

// fn gives_ownership() -> String {             // gives_ownership 會將他的回傳值
//                                              // 移動給呼叫它的函式

//     let some_string = String::from("你的字串"); // some_string 進入作用域

//     some_string                              // 回傳 some_string 並移動給
//                                              // 呼叫它的函式
// }

// // 此函式會取得一個 String 然後回傳它
// fn takes_and_gives_back(a_string: String) -> String { // a_string 進入作用域
//     a_string  // 回傳 a_string 並移動給呼叫的函式
// }

// fn main() {
//     let s1 = String::from("hello");

//     let (s2, len) = calculate_length(s1);

//     println!("'{}' 的長度為 {}。", s2, len);
// }

// fn calculate_length(s: String) -> (String, usize) {
//     let length = s.len(); // len() 回傳 String 的長度

//     (s, length)
// }

// fn main() {
//     let s1 = String::from("hello");

//     let len = calculate_length(&s1);

//     println!("'{}' 的長度為 {}。", s1, len);
// }

// fn calculate_length(s: &String) -> usize {
//     s.len()
// }

// fn main() {
//     let mut s = String::from("hello");

//     let r1 = &s; // 沒問題
//     let r2 = &s; // 沒問題
//     println!("{} and {}", r1, r2);
//     // 變數 r1 和 r2 將不再使用

//     let r3 = &mut s; // 沒問題
//     println!("{}", r3);
// }

// fn first_word(s: &String) -> usize {
//     let bytes = s.as_bytes();

//     for (i, &item) in bytes.iter().enumerate() {
//         if item == b' ' {
//             return i;
//         }
//     }

//     s.len()
// }

// fn first_word(s: &str) -> &str {
//     let bytes = s.as_bytes();

//     for (i, &item) in bytes.iter().enumerate() {
//         if item == b' ' {
//             return &s[0..i];
//         }
//     }

//     &s[..]
// }

// fn main() {
//     // let mut s = String::from("hello world");
//     let s = String::from("hello world");

//     let word = first_word(&s); // word 取得數值 5

//     // s.clear(); // 這會清空 String，這就等於 ""

//     println!("第一個單字為：{}", word);

//     // // word 仍然是數值 5 ，但是我們已經沒有相等意義的字串了
//     // // 擁有 5 的變數 word 現在完全沒意義！

//     // let s = String::from("hello world");

//     // let len = s.len();

//     // // let hello = &s[0..5];
//     // let hello = &s[..5];
//     // // let world = &s[6..11];
//     // let world = &s[6..len];
//     // println!("{} {}", hello, world);
// }

// fn main() {
//     let my_string = String::from("hello world");

//     // first_word 適用於 `String` 的切片，無論是部分或整體
//     let word = first_word(&my_string[0..6]);
//     let word = first_word(&my_string[..]);
//     // first_word 也適用於 `String` 的參考，這等同於對整個 `String` 切片的操作。
//     let word = first_word(&my_string);

//     let my_string_literal = "hello world";

//     // first_word 適用於字串字面值，無論是部分或整體
//     let word = first_word(&my_string_literal[0..6]);
//     let word = first_word(&my_string_literal[..]);

//     // 因為字串字面值本來就是切片
//     // 沒有切片語法也是可行的！
//     let word = first_word(my_string_literal);
// }

#![allow(unused)]
fn main() {
    let a = [1, 2, 3, 4, 5];

    let slice = &a[1..3];

    assert_eq!(slice, &[2, 3]);
}
