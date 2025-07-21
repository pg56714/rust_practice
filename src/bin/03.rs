// const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;

// fn main() {
//     let mut x = 5;
//     println!("x 的數值為：{x}");
//     x = 6;
//     println!("x 的數值為：{x}");

//     println!("三小時的秒數為：{THREE_HOURS_IN_SECONDS}");
// }
// fn main() {
//     let x = 5;

//     let x = x + 1;

//     {
//         let x = x * 2;
//         println!("x 在內部範圍的數值為：{x}");
//     }

//     println!("x 的數值為：{x}");

//     let spaces = "   ";
//     let spaces = spaces.len();
//     println!("空格的長度為：{spaces}");

//     // let mut spaces = "   ";
//     // spaces = spaces.len(); // 這行會報錯，因為 spaces 現在是 usize 類型，不是字串類型(可變變數仍然是無法變更變數型別的)
// }
// fn main() {
//     let c = 'z';
//     let z: char = 'ℤ';
//     let heart_eyed_cat = '😻';
//     println!("字元 c 的值為：{c}");
//     println!("字元 z 的值為：{z}");
//     println!("字元 heart_eyed_cat 的值為：{heart_eyed_cat}");
// }

// type Tuple = (i32, f64, u8);
// fn main() {
//     // let tup: (i32, f64, u8) = (500, 6.4, 1);
//     // let tup = (500, 6.4, 1);
//     let tup: Tuple = (500, 6.4, 1);

//     let (x, y, z) = tup;

//     println!("x 的數值為：{x}");
//     println!("y 的數值為：{y}");
//     println!("z 的數值為：{z}");
//     println!("tup 的第一個元素為：{}", tup.0);
// }

// fn main() {
//     // let a = [1, 2, 3, 4, 5];

//     // i32 在此是每個元素的型別，在分號後面的數字 5 指的是此陣列有五個元素。
//     // let a: [i32; 5] = [1, 2, 3, 4, 5];

//     let a = [3; 5]; // 這會建立一個陣列，裡面有五個元素，每個元素的值都是 3。
//     println!("a 的第一個元素為：{}", a[0]);
//     println!("a 的第二個元素為：{}", a[1]);
// }
// fn main() {
//     println!("Hello, world!");

//     another_function();
// }

// fn another_function() {
//     println!("另一支函式。");
// }
// fn main() {
//     another_function(5);
// }

// fn another_function(x: i32) {
//     println!("x 的數值為：{x}");
// }
// fn main() {
//     print_labeled_measurement(5, 'h');
// }

// fn print_labeled_measurement(value: i32, unit_label: char) {
//     println!("測量值爲：{value}{unit_label}");
// }
// fn main() {
//     // let x: i32 = 5;

//     let y = {
//         let x = 3;
//         x + 1
//     };

//     println!("y 的數值為：{y}");
// }
// fn five() -> i32 {
//     5
// }

// fn main() {
//     let x = five();

//     println!("x 的數值為：{x}");
// }

// fn main() {
//     let x = plus_one(5);

//     println!("x 的數值為：{x}");
// }

// fn plus_one(x: i32) -> i32 {
//     x + 1
// }
// fn main() {
//     let number = 3;

//     if number < 5 {
//         println!("條件為真");
//     } else {
//         println!("條件為否");
//     }
// }

// fn main() {
//     let condition = true;
//     let number = if condition { 5 } else { 6 };

//     println!("數字結果為：{number}");
// }

// fn main() {
//     let mut counter = 0;

//     let result = loop {
//         counter += 1;

//         if counter == 10 {
//             break counter * 2;
//         }
//     };

//     println!("結果為：{result}");
// }

// fn main() {
//     let mut count = 0;
//     'counting_up: loop {
//         println!("count = {count}");
//         let mut remaining = 10;

//         loop {
//             println!("remaining = {remaining}");
//             if remaining == 9 {
//                 break;
//             }
//             if count == 2 {
//                 break 'counting_up;
//             }
//             remaining -= 1;
//         }

//         count += 1;
//     }
//     println!("End count = {count}");
// }
// fn main() {
//     let mut number = 3;

//     while number != 0 {
//         println!("{number}!");

//         number -= 1;
//     }

//     println!("升空！！！");
// }

// fn main() {
//     let a = [10, 20, 30, 40, 50];
//     let mut index = 0;

//     while index < 5 {
//         println!("數值為：{}", a[index]);

//         index += 1;
//     }
// }

// fn main() {
//     let a = [10, 20, 30, 40, 50];

//     for element in a {
//         println!("數值為：{element}");
//     }
// }
// fn main() {
//     for number in (1..4).rev() {
//         println!("{number}!");
//     }
//     println!("升空！！！");
// }

// 轉換攝氏與華氏溫度
// fn main() {
//     let celsius = 30.0;
//     let fahrenheit = celsius_to_fahrenheit(celsius);
//     println!("{celsius} °C = {fahrenheit} °F");

//     let f = 86.0;
//     let c = fahrenheit_to_celsius(f);
//     println!("{f} °F = {c} °C");
// }

// fn celsius_to_fahrenheit(c: f64) -> f64 {
//     c * 9.0 / 5.0 + 32.0
// }

// fn fahrenheit_to_celsius(f: f64) -> f64 {
//     (f - 32.0) * 5.0 / 9.0
// }

// 產生第 n 個斐波那契數字
// fn main() {
//     let n = 10;
//     let result = fibonacci(n);
//     println!("第 {n} 個斐波那契數字是：{result}");
// }

// fn fibonacci(n: u32) -> u32 {
//     if n == 0 {
//         return 0;
//     } else if n == 1 {
//         return 1;
//     }

//     let mut a: u32 = 0;
//     let mut b = 1;

//     for _ in 2..=n {
//         let temp = b;
//         b = a + b;
//         a = temp;
//     }

//     b
// }

// 印出《The Twelve Days of Christmas》歌詞
fn main() {
    let days = [
        "first", "second", "third", "fourth", "fifth", "sixth", "seventh", "eighth", "ninth",
        "tenth", "eleventh", "twelfth",
    ];

    let gifts = [
        "a Partridge in a Pear Tree.",
        "two Turtle Doves,",
        "three French Hens,",
        "four Calling Birds,",
        "five Gold Rings,",
        "six Geese a-Laying,",
        "seven Swans a-Swimming,",
        "eight Maids a-Milking,",
        "nine Ladies Dancing,",
        "ten Lords a-Leaping,",
        "eleven Pipers Piping,",
        "twelve Drummers Drumming,",
    ];

    for day in 0..12 {
        println!(
            "\nOn the {} day of Christmas, my true love gave to me:",
            days[day]
        );

        for gift in (0..=day).rev() {
            if day > 0 && gift == 0 {
                println!("and {}", gifts[gift]);
            } else {
                println!("{}", gifts[gift]);
            }
        }
    }
}
