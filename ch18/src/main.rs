// fn main() {
//     let favorite_color: Option<&str> = None;
//     let is_tuesday = false;
//     let age: Result<u8, _> = "34".parse();

//     if let Some(color) = favorite_color {
//         println!("使用你最喜歡的顏色{color}作為背景");
//     } else if is_tuesday {
//         println!("星期二就用綠色！");
//     } else if let Ok(age) = age {
//         if age > 30 {
//             println!("使用紫色作為背景顏色");
//         } else {
//             println!("使用橘色作為背景顏色");
//         }
//     } else {
//         println!("使用藍色作為背景顏色");
//     }
// }

// fn main() {
//     let mut stack = Vec::new();

//     stack.push(1);
//     stack.push(2);
//     stack.push(3);

//     while let Some(top) = stack.pop() {
//         println!("{}", top);
//     }
// }

// fn main() {
//     let v = vec!['a', 'b', 'c'];

//     for (index, value) in v.iter().enumerate() {
//         println!("{} 位於索引 {}", value, index);
//     }
// }

fn main() {
    enum Message {
        Hello { id: i32 },
    }

    let msg = Message::Hello { id: 5 };

    match msg {
        Message::Hello {
            id: id_variable @ 3..=7,
        } => println!("id 在此範圍中：{id_variable}"),
        Message::Hello { id: 10..=12 } => {
            println!("id 在其他範圍中")
        }
        Message::Hello { id } => println!("找到其他 id：{id}"),
    }
}
