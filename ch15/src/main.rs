// fn main() {
//     let b = Box::new(5);
//     println!("b = {b}");
// }

// enum List {
//     Cons(i32, List),
//     Nil,
// }

// use crate::List::{Cons, Nil};

// fn main() {
//     let list = Cons(1, Cons(2, Cons(3, Nil)));
// }

// enum List {
//     Cons(i32, Box<List>),
//     Nil,
// }

// use crate::List::{Cons, Nil};

// fn main() {
//     let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
// }

// fn main() {
//     let x = 5;
//     // let y = &x;
//     let y = Box::new(x);

//     assert_eq!(5, x);
//     assert_eq!(5, *y);
// }

// use std::ops::Deref;
// struct MyBox<T>(T);

// impl<T> MyBox<T> {
//     fn new(x: T) -> MyBox<T> {
//         MyBox(x)
//     }
// }

// impl<T> Deref for MyBox<T> {
//     type Target = T;

//     fn deref(&self) -> &Self::Target {
//         &self.0
//     }
// }

// // fn main() {
// //     let x = 5;
// //     let y = MyBox::new(x);

// //     assert_eq!(5, x);
// //     assert_eq!(5, *y);
// // }

// fn hello(name: &str) {
//     println!("Hello, {name}!");
// }

// // fn main() {
// //     let m = MyBox::new(String::from("Rust"));
// //     hello(&m);
// // }

// fn main() {
//     let m = MyBox::new(String::from("Rust"));
//     hello(&(*m)[..]);
// }

// struct CustomSmartPointer {
//     data: String,
// }

// impl Drop for CustomSmartPointer {
//     fn drop(&mut self) {
//         println!("釋放 CustomSmartPointer 的資料 `{}`！", self.data);
//     }
// }

// fn main() {
//     // let _c = CustomSmartPointer {
//     //     data: String::from("我的東東"),
//     // };
//     // let _d = CustomSmartPointer {
//     //     data: String::from("其他東東"),
//     // };
//     // println!("CustomSmartPointers 建立完畢。");
//     let c = CustomSmartPointer {
//         data: String::from("某些資料"),
//     };
//     println!("CustomSmartPointer 建立完畢。");
//     // c.drop();
//     drop(c);
//     println!("CustomSmartPointer 在 main 結束前就被釋放了。");
// }

// 無法使用
// enum List {
//     Cons(i32, Box<List>),
//     Nil,
// }

// use crate::List::{Cons, Nil};

// fn main() {
//     let a = Cons(5, Box::new(Cons(10, Box::new(Nil))));
//     let b = Cons(3, Box::new(a));
//     let c = Cons(4, Box::new(a));
// }

// // 解決上面問題
// enum List {
//     #[allow(dead_code)]
//     Cons(i32, Rc<List>),
//     Nil,
// }

// use crate::List::{Cons, Nil};
// use std::rc::Rc;

// // fn main() {
// //     let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
// //     let b = Cons(3, Rc::clone(&a));
// //     let c = Cons(4, Rc::clone(&a));
// // }

// // Rc裡面的數據無法被修改，只可以被共享
// fn main() {
//     let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
//     println!("建立 a 後的計數 = {}", Rc::strong_count(&a));
//     let _b = Cons(3, Rc::clone(&a));
//     println!("建立 b 後的計數 = {}", Rc::strong_count(&a));
//     {
//         let _c = Cons(4, Rc::clone(&a));
//         println!("建立 c 後的計數 = {}", Rc::strong_count(&a));
//     }
//     println!("c 離開作用域後的計數 = {}", Rc::strong_count(&a));
// }

// #[derive(Debug)]
// enum List {
//     #[allow(dead_code)]
//     Cons(Rc<RefCell<i32>>, Rc<List>),
//     Nil,
// }

// use crate::List::{Cons, Nil};
// use std::cell::RefCell;
// use std::rc::Rc;

// fn main() {
//     let value = Rc::new(RefCell::new(5));

//     let a = Rc::new(Cons(Rc::clone(&value), Rc::new(Nil)));

//     let b = Cons(Rc::new(RefCell::new(3)), Rc::clone(&a));
//     let c = Cons(Rc::new(RefCell::new(4)), Rc::clone(&a));

//     *value.borrow_mut() += 10;

//     println!("a 之後 = {a:?}");
//     println!("b 之後 = {b:?}");
//     println!("c 之後 = {c:?}");
// }

// use crate::List::{Cons, Nil};
// use std::cell::RefCell;
// use std::rc::Rc;

// #[derive(Debug)]
// enum List {
//     #[allow(dead_code)]
//     Cons(i32, RefCell<Rc<List>>),
//     Nil,
// }

// impl List {
//     fn tail(&self) -> Option<&RefCell<Rc<List>>> {
//         match self {
//             Cons(_, item) => Some(item),
//             Nil => None,
//         }
//     }
// }

// fn main() {
//     let a = Rc::new(Cons(5, RefCell::new(Rc::new(Nil))));

//     println!("a 初始參考計數 = {}", Rc::strong_count(&a));
//     println!("a 下個項目 = {:?}", a.tail());

//     let b = Rc::new(Cons(10, RefCell::new(Rc::clone(&a))));

//     println!("a 在 b 建立後的參考計數 = {}", Rc::strong_count(&a));
//     println!("b 初始參考計數 = {}", Rc::strong_count(&b));
//     println!("b 下個項目 = {:?}", b.tail());

//     if let Some(link) = a.tail() {
//         *link.borrow_mut() = Rc::clone(&b);
//     }

//     println!("b 在變更 a 後的參考計數 = {}", Rc::strong_count(&b));
//     println!("a 在變更 a 後的參考計數 = {}", Rc::strong_count(&a));

//     // 取消下一行的註解可以看到循環產生
//     // 這會讓堆疊溢位
//     // println!("a 下個項目 = {:?}", a.tail());
// }

// use std::cell::RefCell;
// use std::rc::{Rc, Weak};

// #[derive(Debug)]
// enum List {
//     Cons(i32, RefCell<Weak<List>>),
//     #[allow(dead_code)]
//     Nil,
// }

// #[allow(dead_code)]
// use crate::List::{Cons, Nil};

// fn main() {
//     let a = Rc::new(Cons(5, RefCell::new(Weak::new())));
//     let b = Rc::new(Cons(10, RefCell::new(Rc::downgrade(&a))));

//     println!("a 的參考計數 = {}", Rc::strong_count(&a)); // 1
//     println!("b 的參考計數 = {}", Rc::strong_count(&b)); // 1

//     // 將 a 的 tail 指回 b（使用 Weak）
//     if let Some(link) = match &*a {
//         Cons(_, link_cell) => Some(link_cell),
//         _ => None,
//     } {
//         *link.borrow_mut() = Rc::downgrade(&b);
//     }

//     println!("a 的參考計數（循環後）= {}", Rc::strong_count(&a)); // 還是 1
//     println!("b 的參考計數（循環後）= {}", Rc::strong_count(&b)); // 還是 1
// }

use std::cell::RefCell;
use std::rc::{Rc, Weak};

#[derive(Debug)]
struct Node {
    #[allow(dead_code)]
    value: i32,
    parent: RefCell<Weak<Node>>,
    #[allow(dead_code)]
    children: RefCell<Vec<Rc<Node>>>,
}

fn main() {
    let leaf = Rc::new(Node {
        value: 3,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![]),
    });

    println!(
        "leaf 的強參考 = {}、弱參考 = {}",
        Rc::strong_count(&leaf),
        Rc::weak_count(&leaf),
    );

    {
        let branch = Rc::new(Node {
            value: 5,
            parent: RefCell::new(Weak::new()),
            children: RefCell::new(vec![Rc::clone(&leaf)]),
        });

        *leaf.parent.borrow_mut() = Rc::downgrade(&branch);

        println!(
            "branch 的強參考 = {}、弱參考 = {}",
            Rc::strong_count(&branch),
            Rc::weak_count(&branch),
        );

        println!(
            "leaf 的強參考 = {}、弱參考 = {}",
            Rc::strong_count(&leaf),
            Rc::weak_count(&leaf),
        );
    }

    println!("leaf 的父節點 {:?}", leaf.parent.borrow().upgrade());
    println!(
        "leaf 的強參考 = {}、弱參考 = {}",
        Rc::strong_count(&leaf),
        Rc::weak_count(&leaf),
    );
}
