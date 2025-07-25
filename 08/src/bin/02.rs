fn to_pig_latin(word: &str) -> String {
    println!("原始單字: {}", word);

    let vowels = ['a', 'e', 'i', 'o', 'u'];
    println!("母音表: {:?}", vowels);

    let mut chars = word.chars(); // 建立字元迭代器
    println!("轉成字元迭代器");

    if let Some(first) = chars.next() {
        println!("第一個字元: '{}'", first);

        let first_lower = first.to_ascii_lowercase();
        println!("小寫第一個字元: '{}'", first_lower);

        if vowels.contains(&first_lower) {
            println!("是母音開頭 → 加上 -hay");
            let result = format!("{}-hay", word);
            println!("結果: {}", result);
            result
        } else {
            println!("是子音開頭 → 第一個移到最後，加 -ay");
            let rest: String = chars.collect(); // 剩下的字元組成新字串
            println!("剩下的字元: {}", rest);
            let result = format!("{}-{}ay", rest, first);
            println!("結果: {}", result);
            result
        }
    } else {
        println!("空字串，回傳空字串");
        String::new()
    }
}

fn main() {
    let words = vec!["apple", "first", "rust", ""]; // 測試空字串也加進來
    for word in words {
        println!("\n--- 處理單字 ---");
        let result = to_pig_latin(word);
        println!("轉換結果：{} → {}\n", word, result);
    }
}
