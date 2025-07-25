fn main() {
    // println!("Hello, world!");

    // let v = vec![1, 2, 3, 4, 5];

    // let third: &i32 = &v[2];
    // println!("第三個元素是 {third}");

    // let third: Option<&i32> = v.get(5);
    // match third {
    //     Some(third) => println!("第三個元素是 {third}"),
    //     None => println!("第三個元素並不存在。"),
    // }

    // let v = vec![100, 32, 57];
    // for i in &v {
    //     println!("{i}");
    // }

    // let mut v1 = vec![100, 32, 57];
    // for i in &mut v1 {
    //     *i += 50;
    //     println!("{i}");
    // }

    // enum SpreadsheetCell {
    //     Int(i32),
    //     Float(f64),
    //     Text(String),
    // }

    // let row = vec![
    //     SpreadsheetCell::Int(3),
    //     SpreadsheetCell::Text(String::from("藍色")),
    //     SpreadsheetCell::Float(10.12),
    // ];
    // for cell in row {
    //     match cell {
    //         SpreadsheetCell::Int(i) => println!("整數: {i}"),
    //         SpreadsheetCell::Float(f) => println!("浮點數: {f}"),
    //         SpreadsheetCell::Text(s) => println!("文字: {s}"),
    //     }
    // }

    // // let mut s = String::new();
    // let data = "初始內容";

    // let s = data.to_string();

    // // 此方法也能直接用於字面值上
    // // let s = "初始內容".to_string();
    // println!("字串內容: {s}");

    // let mut s1 = String::from("foo");
    // // s1.push_str("bar");
    // let s2 = "bar";
    // s1.push_str(s2);
    // println!("s1 is {s1}");

    // let s1 = String::from("Hello, ");
    // let s2 = String::from("world!");
    // let s3 = s1 + &s2;
    // println!("s3 is {s3}");

    // let s1 = String::from("tic");
    // let s2 = String::from("tac");
    // let s3 = String::from("toe");

    // // let s = s1 + "-" + &s2 + "-" + &s3;
    // let s = format!("{s1}-{s2}-{s3}"); // 跟 println! 一樣，format! 會返回一個新的 String，而不會改變原有的 String。
    // println!("s is {s}");

    // for c in "Зд".chars() {
    //     println!("{c}");
    // }

    // for b in "Зд".bytes() {
    //     println!("{b}");
    // }

    // use std::collections::HashMap;

    // let mut scores = HashMap::new();

    // scores.insert(String::from("藍隊"), 10);
    // scores.insert(String::from("黃隊"), 50);
    // // let team_name = String::from("藍隊");
    // // let score = scores.get(&team_name).copied().unwrap_or(0);
    // // // 使用 `copied()` 來獲取值的副本，並使用 `unwrap_or(0)` 來處理可能的 None 值
    // // println!("{} 的分數是 {}", team_name, score);
    // for (key, value) in &scores {
    //     println!("{key}: {value}");
    // }

    // use std::collections::HashMap;

    // let field_name = String::from("Favorite color");
    // let field_value = String::from("藍隊");

    // let mut map = HashMap::new();
    // map.insert(field_name, field_value);
    // // field_name 和 field_value 在這之後就不能使用了，你可以試著使用它們並看看編譯器回傳什麼錯誤

    // use std::collections::HashMap;

    // let mut scores = HashMap::new();

    // // scores.insert(String::from("藍隊"), 10);
    // // scores.insert(String::from("藍隊"), 25);
    // scores.insert(String::from("藍隊"), 10);

    // scores.entry(String::from("黃隊")).or_insert(50);
    // scores.entry(String::from("藍隊")).or_insert(50);

    // println!("{scores:?}");

    use std::collections::HashMap;

    let text = "hello world wonderful world";

    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }

    println!("{:?}", map);
}
