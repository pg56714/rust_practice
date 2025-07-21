// struct User {
//     active: bool,
//     username: String,
//     email: String,
//     sign_in_count: u64,
// }

// fn build_user(email: String, username: String) -> User {
//     User {
//         active: true,
//         email,
//         username,
//         sign_in_count: 1,
//     }
// }

// fn main() {
//     // let mut user1 = User {
//     //     active: true,
//     //     email: String::from("someone@example.com"),
//     //     username: String::from("someusername123"),
//     //     sign_in_count: 1,
//     // };

//     // user1.email = String::from("anotheremail@example.com");

//     // let user2 = User {
//     //     active: user1.active,
//     //     username: user1.username,
//     //     email: String::from("another@example.com"),
//     //     sign_in_count: user1.sign_in_count,
//     // };
//     // // let user2 = User {
//     // //     email: String::from("another@example.com"),
//     // //     ..user1
//     // // };

//     // // println!("User 1: {}, {}, {}", user1.username, user1.email, user1.sign_in_count);
//     // println!("User 2: {}, {}, {}", user2.username, user2.email, user2.sign_in_count);

//     let user1 = build_user(
//         String::from("someone@example.com"),
//         String::from("someusername123"),
//     );
//     println!(
//         "User 1: {}, {}, {}, {}",
//         user1.active, user1.username, user1.email, user1.sign_in_count
//     );
// }

// struct Color(i32, i32, i32);
// struct Point(i32, i32, i32);

// fn main() {
//     let black = Color(0, 0, 0);
//     let origin = Point(0, 0, 0);
//     println!("Black: ({}, {}, {}), Origin: ({}, {}, {})",
//              black.0, black.1, black.2,
//              origin.0, origin.1, origin.2);
// }

// struct AlwaysEqual;

// fn main() {
//     let subject = AlwaysEqual;
// }

// fn main() {
//     let width1 = 30;
//     let height1 = 50;

//     println!(
//         "長方形的面積為 {} 平方像素。",
//         area(width1, height1)
//     );
// }

// fn area(width: u32, height: u32) -> u32 {
//     width * height
// }

// fn main() {
//     let rect1 = (30, 50);

//     println!(
//         "長方形的面積為 {} 平方像素。",
//         area(rect1)
//     );
// }

// fn area(dimensions: (u32, u32)) -> u32 {
//     dimensions.0 * dimensions.1
// }

// struct Rectangle {
//     width: u32,
//     height: u32,
// }

// fn main() {
//     let rect1 = Rectangle {
//         width: 30,
//         height: 50,
//     };

//     println!(
//         "長方形的面積為 {} 平方像素。",
//         area(&rect1)
//     );
// }

// fn area(rectangle: &Rectangle) -> u32 {
//     rectangle.width * rectangle.height
// }

// #[derive(Debug)]
// struct Rectangle {
//     width: u32,
//     height: u32,
// }

// fn main() {
//     // let rect1 = Rectangle {
//     //     width: 30,
//     //     height: 50,
//     // };

//     // // println!("rect1 is {:?}", rect1);
//     // println!("rect1 is {rect1:#?}");

//     let scale = 2;
//     let rect1 = Rectangle {
//         width: dbg!(30 * scale),
//         height: 50,
//     };

//     dbg!(&rect1);
// }

// #[derive(Debug)]
// struct Rectangle {
//     width: u32,
//     height: u32,
// }

// impl Rectangle {
//     fn area(&self) -> u32 {
//         self.width * self.height
//     }
// }

// fn main() {
//     let rect1 = Rectangle {
//         width: 30,
//         height: 50,
//     };

//     println!(
//         "長方形的面積為 {} 平方像素。",
//         rect1.area()
//     );
// }

// impl Rectangle {
//     fn width(&self) -> bool {
//         self.width > 0
//     }
// }

// fn main() {
//     let rect1 = Rectangle {
//         width: 30,
//         height: 50,
//     };

//     if rect1.width() {
//         println!("長方形的寬度不為零，而是 {}", rect1.width);
//     }
// }

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    // fn area(&self) -> u32 {
    //     self.width * self.height
    // }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };
    let rect3 = Rectangle {
        width: 60,
        height: 45,
    };

    println!("rect1 能容納 rect2 嗎？{}", rect1.can_hold(&rect2));
    println!("rect1 能容納 rect3 嗎？{}", rect1.can_hold(&rect3));

    let square = Rectangle::square(3);
    println!("正方形的寬度和高度都是：{}", square.width);
}
