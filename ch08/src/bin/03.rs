use std::collections::HashMap;
use std::io::{self, Write};

fn main() {
    let mut company: HashMap<String, Vec<String>> = HashMap::new();

    loop {
        print!("請輸入指令（如：將 莎莉 加入 工程部門 / 查詢 工程部門 / 全部查詢 / exit）：");
        io::stdout().flush().unwrap(); // flush 會確保提示符號立即顯示、unwrap() 保證這裡一定有值，如果不是，就讓程式 panic

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        let trimmed = input.trim();
        if trimmed == "exit" {
            break;
        }

        let tokens: Vec<&str> = trimmed.split_whitespace().collect();

        // ➤ 新增員工到部門
        if tokens.len() == 4 && tokens[0] == "將" && tokens[2] == "加入" {
            let name = tokens[1].to_string();
            let dept = tokens[3].to_string();
            company.entry(dept).or_default().push(name);
            println!("✅ 已加入");
        }
        // ➤ 查詢部門員工
        else if tokens.len() == 2 && tokens[0] == "查詢" {
            let dept = tokens[1];
            if let Some(employees) = company.get(dept) {
                let mut sorted = employees.clone();
                sorted.sort();
                println!("{} 部門員工：{:?}", dept, sorted);
            } else {
                println!("⚠️ 找不到該部門");
            }
        }
        // ➤ 全部查詢：列出所有部門員工（依部門名排序）
        else if trimmed == "全部查詢" {
            let mut depts: Vec<_> = company.keys().collect();
            depts.sort();

            let mut all_employees = Vec::new();
            for dept in depts {
                if let Some(list) = company.get(dept) {
                    all_employees.extend_from_slice(list);
                }
            }
            all_employees.sort();
            println!("公司所有員工（依姓名排序）：{:?}", all_employees);
        }
        // ➤ 格式錯誤
        else {
            println!("❌ 格式錯誤，請重新輸入。");
        }
    }
}
