// fn main() {
//     let number_list = vec![34, 50, 25, 100, 65];

//     let mut largest = &number_list[0];

//     for number in &number_list {
//         if number > largest {
//             largest = number;
//         }
//     }

//     println!("最大數字為 {}", largest);
//     assert_eq!(*largest, 100);
// }

// fn largest_i32(list: &[i32]) -> &i32 {
//     let mut largest = &list[0];

//     for item in list {
//         if item > largest {
//             largest = item;
//         }
//     }

//     largest
// }

// fn largest_char(list: &[char]) -> &char {
//     let mut largest = &list[0];

//     for item in list {
//         if item > largest {
//             largest = item;
//         }
//     }

//     largest
// }

// fn main() {
//     let number_list = vec![34, 50, 25, 100, 65];

//     let result = largest_i32(&number_list);
//     println!("最大數字為 {}", result);
//     assert_eq!(*result, 100);

//     let char_list = vec!['y', 'm', 'a', 'q'];

//     let result = largest_char(&char_list);
//     println!("最大字元為 {}", result);
//     assert_eq!(*result, 'y');
// }

// fn largest<T: PartialOrd>(list: &[T]) -> &T {
//     let mut largest = &list[0];

//     for item in list {
//         if item > largest {
//             largest = item;
//         }
//     }

//     largest
// }

// fn main() {
//     let number_list = vec![34, 50, 25, 100, 65];

//     let result = largest(&number_list);
//     println!("最大數字為 {result}");

//     let char_list = vec!['y', 'm', 'a', 'q'];

//     let result = largest(&char_list);
//     println!("最大字元為 {result}");
// }

// struct Point<T, U> {
//     x: T,
//     y: U,
// }

// fn main() {
//     let both_integer = Point { x: 5, y: 10 };
//     let both_float = Point { x: 1.0, y: 4.0 };
//     let integer_and_float = Point { x: 5, y: 4.0 };
// }

// struct Point<T> {
//     x: T,
//     y: T,
// }

// impl<T> Point<T> {
//     fn x(&self) -> &T {
//         &self.x
//     }
// }

// fn main() {
//     let p = Point { x: 5, y: 10 };

//     println!("p.x = {}", p.x());
//     println!("p.y = {}", p.y);
// }

// impl Point<f32> {
//     fn distance_from_origin(&self) -> f32 {
//         (self.x.powi(2) + self.y.powi(2)).sqrt()
//     }
// }

// struct Point<X1, Y1> {
//     x: X1,
//     y: Y1,
// }

// impl<X1, Y1> Point<X1, Y1> {
//     fn mixup<X2, Y2>(self, other: Point<X2, Y2>) -> Point<X1, Y2> {
//         Point {
//             x: self.x,
//             y: other.y,
//         }
//     }
// }

// fn main() {
//     let p1 = Point { x: 5, y: 10.4 };
//     let p2 = Point { x: "Hello", y: 'c' };

//     let p3 = p1.mixup(p2);

//     println!("p3.x = {}, p3.y = {}", p3.x, p3.y);
// }

// pub trait Summary {
//     fn summarize(&self) -> String;
// }

// struct NewsArticle {
// // pub struct NewsArticle {
//     pub headline: String,
//     pub location: String,
//     pub author: String,
//     pub content: String,
// }

// impl Summary for NewsArticle {
//     fn summarize(&self) -> String {
//         format!("{} {} 著 ({})", self.headline, self.author, self.location)
//     }
//     // fn summarize(&self) -> String {
//     //     String::from("(閱讀更多...)")
//     // }
// }

// struct Tweet {
// // pub struct Tweet {
//     pub username: String,
//     pub content: String,
//     pub reply: bool,
//     pub retweet: bool,
// }

// impl Summary for Tweet {
//     fn summarize(&self) -> String {
//         format!("{}: {}", self.username, self.content)
//     }
// }

// // use aggregator::{self, Summary, Tweet};

// // pub fn notify(item: &impl Summary) {
// //     println!("頭條新聞！{}", item.summarize());
// // }
// pub fn notify<T: Summary>(item: &T) {
//     println!("頭條新聞！{}", item.summarize());
// }

// fn main() {
//     let tweet = Tweet {
//         username: String::from("horse_ebooks"),
//         content: String::from(
//             "of course, as you probably already know, people",
//         ),
//         reply: false,
//         retweet: false,
//     };

//     println!("1 則新推文：{}", tweet.summarize());

//     let article = NewsArticle {
//         headline: String::from("Penguins win the Stanley Cup Championship!"),
//         location: String::from("Pittsburgh, PA, USA"),
//         author: String::from("Iceburgh"),
//         content: String::from(
//             "The Pittsburgh Penguins once again are the best \
//              hockey team in the NHL.",
//         ),
//     };

//     println!("有新文章發佈！{}", article.summarize());
// }

// fn some_function<T: Display + Clone, U: Clone + Debug>(t: &T, u: &U) -> i32 {
// 等於
// fn some_function<T, U>(t: &T, u: &U) -> i32
// where
//     T: Display + Clone,
//     U: Clone + Debug,
// {

// use std::fmt::Display;

// struct Pair<T> {
//     x: T,
//     y: T,
// }

// impl<T> Pair<T> {
//     fn new(x: T, y: T) -> Self {
//         Self { x, y }
//     }
// }

// impl<T: Display + PartialOrd> Pair<T> {
//     fn cmp_display(&self) {
//         if self.x >= self.y {
//             println!("最大的是 x = {}", self.x);
//         } else {
//             println!("最大的是 y = {}", self.y);
//         }
//     }
// }

// fn main() {
//     let pair = Pair::new(5, 10);
//     pair.cmp_display();

//     let pair = Pair::new(1.0, 4.0);
//     pair.cmp_display();

//     let pair = Pair::new("Hello", "World");
//     pair.cmp_display();
// }

// 生命週期標註
// &i32        // 一個參考
// &'a i32     // 一個有顯式生命週期的參考
// &'a mut i32 // 一個有顯式生命週期的可變參考
// fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
//     if x.len() > y.len() {
//         x
//     } else {
//         y
//     }
// }

// fn main() {
//     let string1 = String::from("abcd");
//     let string2 = "xyz";

//     let result = longest(string1.as_str(), string2);
//     println!("最長的字串為 {}", result);
// }

// struct ImportantExcerpt<'a> {
//     part: &'a str,
// }

// fn main() {
//     let novel = String::from("Call me Ishmael. Some years ago...");
//     let first_sentence = novel.split('.').next().expect("無法找到 '.'");
//     let i = ImportantExcerpt {
//         part: first_sentence,
//     };
//     println!("重要摘錄: {}", i.part);
// }

fn main() {
    let string1 = String::from("abcd");
    let string2 = "xyz";

    let result =
        longest_with_an_announcement(string1.as_str(), string2, "Today is someone's birthday!");
    println!("最長的字串為 {}", result);
}

use std::fmt::Display;

fn longest_with_an_announcement<'a, T>(x: &'a str, y: &'a str, ann: T) -> &'a str
where
    T: Display,
{
    println!("公告！{}", ann);
    if x.len() > y.len() { x } else { y }
}
