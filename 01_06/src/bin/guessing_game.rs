use std::io;
// io 輸入／輸出（input/output）函式庫引入作用域中。 io 函式庫來自標準函式庫（常稱為 std）
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("請猜測一個數字！");

    let mut rng = rand::rng();
    let secret_number = rng.random_range(1..=100);

    // if cfg!(debug_assertions) {
    //     // 如果是 debug 模式，則顯示祕密數字
    //     println!("祕密數字為：{secret_number}");
    // }

    // 這樣寫編譯後的程式不會包含這行程式碼。
    #[cfg(debug_assertions)]
    println!("祕密數字為：{secret_number}");

    loop {
        println!("請輸入你的猜測數字。");

        // let apple = 5; // 不可變的
        // let mut banana = 5; // 可變的
        let mut guess = String::new();

        io::stdin().read_line(&mut guess).expect("讀取該行失敗");

        // let guess: u32 = guess.trim().parse().expect("請輸入一個數字！");
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                eprint!("請輸入一個有效的數字！\n");
                continue;
            }
        };

        println!("你的猜測數字：{guess}");

        // let x = 5;
        // let y = 10;
        // println!("x = {x} 而且 y + 2 = {}", y + 2);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("太小了！"),
            Ordering::Greater => println!("太大了！"),
            // Ordering::Equal => println!("獲勝！"),
            Ordering::Equal => {
                println!("獲勝！");
                break;
            }
        }
    }
}
