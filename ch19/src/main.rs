// fn main() {
//     // 允許忽略借用規則，同時可存在指向相同位置的可變和不可變的指標，或是多個可變指標
//     // 不能保證一定指向有效記憶體
//     // 可以為空（null）
//     // 並無實作任何自動清理機制
//     let mut num = 5;

//     let r1 = &num as *const i32;
//     let r2 = &mut num as *mut i32;

//     unsafe {
//         println!("r1 is: {}", *r1);
//         println!("r2 is: {}", *r2);
//     }

//     let num2 = unsafe { *r2 };
//     println!("num2 is: {}", num2);

//     // let address = 0x012345usize;
//     // let r = address as *const i32;

//     unsafe fn dangerous() {}

//     unsafe {
//         dangerous();
//     }
// }

// fn main() {
//     let mut v = vec![1, 2, 3, 4, 5, 6];

//     let r = &mut v[..];

//     let (a, b) = r.split_at_mut(3);

//     assert_eq!(a, &mut [1, 2, 3]);
//     assert_eq!(b, &mut [4, 5, 6]);
// }

// unsafe extern "C" {
//     unsafe fn abs(input: i32) -> i32;
// }

// fn main() {
//     unsafe {
//         println!("依據 C 所判斷 -3 的絕對值為：{}", abs(-3));
//     }
// }

// static HELLO_WORLD: &str = "Hello, world!";

// fn main() {
//     println!("name 為：{HELLO_WORLD}");
// }

// use std::ptr;

// static mut COUNTER: u32 = 0;

// fn add_to_count(inc: u32) {
//     unsafe {
//         COUNTER += inc;
//     }
// }

// fn main() {
//     add_to_count(3);

//     unsafe {
//         let val = ptr::read_volatile(&raw const COUNTER);
//         println!("COUNTER: {val}");
//     }
// }

// use std::ops::Add;

// #[derive(Debug, Copy, Clone, PartialEq)]
// struct Point {
//     x: i32,
//     y: i32,
// }

// impl Add for Point {
//     type Output = Point;

//     fn add(self, other: Point) -> Point {
//         Point {
//             x: self.x + other.x,
//             y: self.y + other.y,
//         }
//     }
// }

// fn main() {
//     assert_eq!(
//         Point { x: 1, y: 0 } + Point { x: 2, y: 3 },
//         Point { x: 3, y: 3 }
//     );
// }

// use std::ops::Add;

// struct Millimeters(u32);
// struct Meters(u32);

// impl Add<Meters> for Millimeters {
//     type Output = Millimeters;

//     fn add(self, other: Meters) -> Millimeters {
//         Millimeters(self.0 + (other.0 * 1000))
//     }
// }

// trait Pilot {
//     fn fly(&self);
// }

// trait Wizard {
//     fn fly(&self);
// }

// struct Human;

// impl Pilot for Human {
//     fn fly(&self) {
//         println!("這裡是艦長發言。");
//     }
// }

// impl Wizard for Human {
//     fn fly(&self) {
//         println!("起！");
//     }
// }

// impl Human {
//     fn fly(&self) {
//         println!("*狂揮雙臂*");
//     }
// }

// fn main() {
//     let person = Human;
//     Pilot::fly(&person);
//     Wizard::fly(&person);
//     person.fly();
// }

// trait Animal {
//     fn baby_name() -> String;
// }

// struct Dog;

// #[allow(dead_code)]
// impl Dog {
//     fn baby_name() -> String {
//         String::from("小不點")
//     }
// }

// impl Animal for Dog {
//     fn baby_name() -> String {
//         String::from("小狗狗")
//     }
// }

// fn main() {
//     // println!("幼犬被稱作{}", Dog::baby_name());
//     println!("幼犬被稱作{}", <Dog as Animal>::baby_name());
// }

// use std::fmt;

// struct Point {
//     x: i32,
//     y: i32,
// }

// impl fmt::Display for Point {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         write!(f, "({}, {})", self.x, self.y)
//     }
// }

// impl OutlinePrint for Point {}

// trait OutlinePrint: fmt::Display {
//     fn outline_print(&self) {
//         let output = self.to_string();
//         let len = output.len();
//         println!("{}", "*".repeat(len + 4));
//         println!("*{}*", " ".repeat(len + 2));
//         println!("* {} *", output);
//         println!("*{}*", " ".repeat(len + 2));
//         println!("{}", "*".repeat(len + 4));
//     }
// }

// fn main() {
//     let point = Point { x: 3, y: 4 };
//     println!("點的位置是：{}", point);
//     point.outline_print();
// }

// use std::fmt;

// struct Wrapper(Vec<String>);

// impl fmt::Display for Wrapper {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         write!(f, "[{}]", self.0.join(", "))
//     }
// }

// fn main() {
//     let w = Wrapper(vec![String::from("hello"), String::from("world")]);
//     println!("w = {}", w);
// }

// fn main() {
//     let f: Box<dyn Fn() + Send + 'static> = Box::new(|| println!("嗨"));

//     fn takes_long_type(f: Box<dyn Fn() + Send + 'static>) {
//         // --省略--
//     }

//     fn returns_long_type() -> Box<dyn Fn() + Send + 'static> {
//         // --省略--
//         Box::new(|| ())
//     }
// }
// fn main() {
//     type Thunk = Box<dyn Fn() + Send + 'static>;

//     let f: Thunk = Box::new(|| println!("嗨"));

//     fn takes_long_type(f: Thunk) {
//         // --省略--
//     }

//     fn returns_long_type() -> Thunk {
//         // --省略--
//         Box::new(|| ())
//     }
// }

// use std::fmt;
// use std::io::Error;

// pub trait Write {
//     fn write(&mut self, buf: &[u8]) -> Result<usize, Error>;
//     fn flush(&mut self) -> Result<(), Error>;

//     fn write_all(&mut self, buf: &[u8]) -> Result<(), Error>;
//     fn write_fmt(&mut self, fmt: fmt::Arguments) -> Result<(), Error>;
// }

// type Result<T> = std::result::Result<T, std::io::Error>;
// use std::fmt;
// use std::io::Error;

// pub trait Write {
//     fn write(&mut self, buf: &[u8]) -> Result<usize, Error>;
//     fn flush(&mut self) -> Result<(), Error>;

//     fn write_all(&mut self, buf: &[u8]) -> Result<(), Error>;
//     fn write_fmt(&mut self, fmt: fmt::Arguments) -> Result<(), Error>;
// }

// fn add_one(x: i32) -> i32 {
//     x + 1
// }

// fn do_twice(f: fn(i32) -> i32, arg: i32) -> i32 {
//     f(arg) + f(arg)
// }

// fn main() {
//     let answer = do_twice(add_one, 5);

//     println!("答案是：{}", answer);
// }

use hello_macro::HelloMacro;
use hello_macro_derive::HelloMacro;

#[derive(HelloMacro)]
struct Pancakes;

fn main() {
    Pancakes::hello_macro();
}
