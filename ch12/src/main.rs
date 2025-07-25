// use std::env;
// use std::fs;

// fn main() {
//     let args: Vec<String> = env::args().collect();

//     let query = &args[1];
//     let file_path = &args[2];

//     println!("搜尋 {query}");
//     println!("目標檔案為 {file_path}");

//     let contents = fs::read_to_string(file_path)
//         .expect("應該要能夠讀取檔案");

//     println!("文字內容：\n{contents}");
// }

// main.rs
// 透過引數數值呼叫命令列解析邏輯
// 設置任何其他的配置
// 呼叫 lib.rs 中的 run 函式
// 如果 run 回傳錯誤的話，處理該錯誤

use std::env;
// use std::fs;
use std::process;
// use std::error::Error;
use ch12::Config;

fn main() {
    let args: Vec<String> = env::args().collect();

    // let config = parse_config(&args);
    // let config = Config::new(&args);
    let config = Config::build(&args).unwrap_or_else(|err| {
        eprintln!("解析引數時出現問題：{err}");
        process::exit(1);
    });

    // println!("搜尋 {}", config.query);
    // println!("目標檔案為 {}", config.file_path);

    // let contents = fs::read_to_string(config.file_path)
    //     .expect("應該要能夠讀取檔案");

    // println!("文字內容：\n{contents}");
    // run(config);
    if let Err(e) = ch12::run(config) {
        // if let Err(e) = run(config) {
        eprintln!("應用程式錯誤：{e}");
        process::exit(1);
    }
}

// // fn run(config: Config) {
// //     let contents = fs::read_to_string(config.file_path)
// //         .expect("應該要能夠讀取檔案");

// //     println!("文字內容：\n{contents}");
// // }

// fn run(config: Config) -> Result<(), Box<dyn Error>> {
//     let contents = fs::read_to_string(config.file_path)?;

//     println!("文字內容：\n{contents}");

//     Ok(())
// }

// struct Config {
//     query: String,
//     file_path: String,
// }

// impl Config {
//     fn build(args: &[String]) -> Result<Config, &'static str> {
//         // if args.len() < 3 {
//         //     panic!("引數不足");
//         // }
//         if args.len() < 3 {
//             return Err("引數不足");
//         }

//         let query = args[1].clone();
//         let file_path = args[2].clone();

//         Ok(Config { query, file_path })
//     }
// }

// // fn parse_config(args: &[String]) -> Config {
// //     // 盡量不要用 clone()，除非必要，第十三章會介紹更好的方法
// //     let query = args[1].clone();
// //     let file_path = args[2].clone();

// //     Config { query, file_path }
// // }
