// pub trait Draw {
//     fn draw(&self);
// }

// pub struct Button {
//     pub width: u32,
//     pub height: u32,
//     pub label: String,
// }

// impl Draw for Button {
//     fn draw(&self) {
//         println!("Draw a Button: label = {}", self.label);
//     }
// }

// pub struct SelectBox {
//     pub width: u32,
//     pub height: u32,
//     pub options: Vec<String>,
// }

// impl Draw for SelectBox {
//     fn draw(&self) {
//         println!("Draw a SelectBox with options: {:?}", self.options);
//     }
// }

// pub struct Screen {
//     pub components: Vec<Box<dyn Draw>>,
// }

// impl Screen {
//     pub fn run(&self) {
//         for component in self.components.iter() {
//             component.draw();
//         }
//     }
// }

// fn main() {
//     let screen = Screen {
//         components: vec![
//             Box::new(SelectBox {
//                 width: 75,
//                 height: 10,
//                 options: vec![
//                     String::from("Yes"),
//                     String::from("Maybe"),
//                     String::from("No"),
//                 ],
//             }),
//             Box::new(Button {
//                 width: 50,
//                 height: 10,
//                 label: String::from("OK"),
//             }),
//         ],
//     };

//     screen.run();
// }

// use ch17::Post;

// fn main() {
//     let mut post = Post::new();
//     post.add_text("我今天午餐吃了沙拉");

//     assert_eq!("", post.content());

//     post.request_review();
//     assert_eq!("", post.content());

//     post.approve();
//     assert_eq!("我今天午餐吃了沙拉", post.content());
// }

use ch17::Post;

fn main() {
    let mut post = Post::new();

    post.add_text("我今天午餐吃了沙拉");
    assert_eq!("", post.content());

    post.request_review();
    assert_eq!("", post.content());

    post.approve();
    assert_eq!("我今天午餐吃了沙拉", post.content());
}
