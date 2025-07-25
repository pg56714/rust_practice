// use std::fs::File;
// use std::io::ErrorKind;

// fn main() {
//     // panic!("◢▆▅▄▃ 崩╰(〒皿〒)╯潰▃▄▅▆◣");
//     let greeting_file_result = File::open("hello.txt");

//     let _greeting_file = match greeting_file_result {
//         Ok(file) => file,
//         Err(error) => match error.kind() {
//             ErrorKind::NotFound => match File::create("hello.txt") {
//                 Ok(fc) => fc,
//                 Err(e) => panic!("建立檔案時發生問題：{:?}", e),
//             },
//             other_error => {
//                 panic!("開啟檔案時發生問題：{:?}", other_error);
//             }
//         },
//     };
// }

// use std::fs::File;
// use std::io::ErrorKind;

// fn main() {
//     let _greeting_file = File::open("hello.txt").unwrap_or_else(|error| {
//         if error.kind() == ErrorKind::NotFound {
//             File::create("hello.txt").unwrap_or_else(|error| {
//                 panic!("建立檔案時發生問題：{:?}", error);
//             })
//         } else {
//             panic!("開啟檔案時發生問題：{:?}", error);
//         }
//     });
// }

// use std::fs::File;

// fn main() {
//     let _greeting_file = File::open("hello.txt").unwrap();
// }

// use std::fs::File;

// fn main() {
//     let _greeting_file = File::open("hello.txt")
//         .expect("hello.txt 應該要存在此專案中");
// }

// #![allow(unused)]
// fn main() {
//     use std::fs::File;
//     use std::io::{self, Read};

//     fn read_username_from_file() -> Result<String, io::Error> {
//         let username_file_result = File::open("hello.txt");

//         let mut username_file = match username_file_result {
//             Ok(file) => file,
//             Err(e) => return Err(e),
//         };

//         let mut username = String::new();

//         match username_file.read_to_string(&mut username) {
//             Ok(_) => Ok(username),
//             Err(e) => Err(e),
//         }
//     }
// }

// // 簡化上面
// use std::fs::File;
// use std::io::{self, Read};

// fn main() {
//     fn _read_username_from_file() -> Result<String, io::Error> {
//         // let mut username_file = File::open("hello.txt")?;
//         // let mut username = String::new();
//         // username_file.read_to_string(&mut username)?;
//         // Ok(username)

//         let mut username = String::new();

//         File::open("hello.txt")?.read_to_string(&mut username)?;

//         Ok(username)
//     }
// }

// // 簡化上面
// use std::fs;
// use std::io;

// fn main() {
//     fn _read_username_from_file() -> Result<String, io::Error> {
//         fs::read_to_string("hello.txt")
//     }
// }

use std::error::Error;
use std::fs::File;

fn main() -> Result<(), Box<dyn Error>> {
    let _greeting_file = File::open("hello.txt")?;

    Ok(())
}
