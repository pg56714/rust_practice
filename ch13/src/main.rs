// #[derive(Debug, PartialEq, Copy, Clone)]
// enum ShirtColor {
//     Red,
//     Blue,
// }

// struct Inventory {
//     shirts: Vec<ShirtColor>,
// }

// impl Inventory {
//     fn giveaway(&self, user_preference: Option<ShirtColor>) -> ShirtColor {
//         user_preference.unwrap_or_else(|| self.most_stocked())
//     }

//     fn most_stocked(&self) -> ShirtColor {
//         let mut num_red = 0;
//         let mut num_blue = 0;

//         for color in &self.shirts {
//             match color {
//                 ShirtColor::Red => num_red += 1,
//                 ShirtColor::Blue => num_blue += 1,
//             }
//         }
//         if num_red > num_blue {
//             ShirtColor::Red
//         } else {
//             ShirtColor::Blue
//         }
//     }
// }

// fn main() {
//     let store = Inventory {
//         shirts: vec![ShirtColor::Blue, ShirtColor::Red, ShirtColor::Blue],
//     };

//     let user_pref1 = Some(ShirtColor::Red);
//     let giveaway1 = store.giveaway(user_pref1);
//     println!(
//         "偏好 {:?} 的使用者獲得 {:?}",
//         user_pref1, giveaway1
//     );

//     let user_pref2 = None;
//     let giveaway2 = store.giveaway(user_pref2);
//     println!(
//         "偏好 {:?} 的使用者獲得 {:?}",
//         user_pref2, giveaway2
//     );
// }

// fn main() {
//     let list = vec![1, 2, 3];
//     println!("定義閉包前：{list:?}");

//     let only_borrows = || println!("來自閉包：{list:?}");

//     println!("呼叫閉包前：{list:?}");
//     only_borrows();
//     println!("呼叫閉包後：{list:?}");
// }

// fn main() {
//     let mut list = vec![1, 2, 3];
//     println!("呼叫閉包前{list:?}");

//     let mut borrows_mutably = || list.push(7);

//     borrows_mutably();
//     println!("呼叫閉包後：{list:?}");
// }

// use std::thread;

// fn main() {
//     let list = vec![1, 2, 3];
//     println!("呼叫閉包前：{list:?}");

//     thread::spawn(move || println!("來自執行緒：{list:?}"))
//         .join()
//         .unwrap();
// }

// #[derive(Debug)]
// struct Rectangle {
//     width: u32,
//     #[allow(dead_code)]
//     height: u32,
// }

// fn main() {
//     let mut list = [
//         Rectangle { width: 10, height: 1 },
//         Rectangle { width: 3, height: 5 },
//         Rectangle { width: 7, height: 12 },
//     ];

//     list.sort_by_key(|r| r.width);
//     println!("{list:#?}");
// }

// #[derive(Debug)]
// struct Rectangle {
//     width: u32,
//     #[allow(dead_code)]
//     height: u32,
// }

// fn main() {
//     let mut list = [
//         Rectangle { width: 10, height: 1 },
//         Rectangle { width: 3, height: 5 },
//         Rectangle { width: 7, height: 12 },
//     ];

//     let mut num_sort_operations = 0;
//     list.sort_by_key(|r| {
//         num_sort_operations += 1;
//         r.width
//     });
//     println!("{list:#?} 的排序經過 {num_sort_operations} 次運算");
// }

// fn main() {
//     let v1 = vec![1, 2, 3];

//     let v1_iter = v1.iter();

//     for val in v1_iter {
//         println!("取得：{val}");
//     }
// }

// #[cfg(test)]
// mod tests {
//     #[test]
//     fn iterator_demonstration() {
//         let v1 = vec![1, 2, 3];

//         let mut v1_iter = v1.iter();

//         assert_eq!(v1_iter.next(), Some(&1));
//         assert_eq!(v1_iter.next(), Some(&2));
//         assert_eq!(v1_iter.next(), Some(&3));
//         assert_eq!(v1_iter.next(), None);
//     }

//     #[test]
//     fn iterator_sum() {
//         let v1 = vec![1, 2, 3];

//         let v1_iter = v1.iter();

//         let total: i32 = v1_iter.sum();

//         assert_eq!(total, 6);
//     }
// }

fn main() {
    let v1: Vec<i32> = vec![1, 2, 3];

    let v2: Vec<_> = v1.iter().map(|x| x + 1).collect();

    assert_eq!(v2, vec![2, 3, 4]);
}

#[derive(PartialEq, Debug)]
#[allow(dead_code)]
struct Shoe {
    size: u32,
    style: String,
}

#[allow(dead_code)]
fn shoes_in_size(shoes: Vec<Shoe>, shoe_size: u32) -> Vec<Shoe> {
    shoes.into_iter().filter(|s| s.size == shoe_size).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn filters_by_size() {
        let shoes = vec![
            Shoe {
                size: 10,
                style: String::from("運動鞋"),
            },
            Shoe {
                size: 13,
                style: String::from("涼鞋"),
            },
            Shoe {
                size: 10,
                style: String::from("靴子"),
            },
        ];

        let in_my_size = shoes_in_size(shoes, 10);

        assert_eq!(
            in_my_size,
            vec![
                Shoe {
                    size: 10,
                    style: String::from("運動鞋")
                },
                Shoe {
                    size: 10,
                    style: String::from("靴子")
                },
            ]
        );
    }
}
