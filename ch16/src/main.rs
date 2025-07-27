// use std::thread;
// use std::time::Duration;

// fn main() {
//     // thread::spawn(|| {
//     //     for i in 1..10 {
//     //         println!("數字 {i} 出現在產生的執行緒中！");
//     //         thread::sleep(Duration::from_millis(1));
//     //     }
//     // });

//     // for i in 1..5 {
//     //     println!("數字 {i} 出現在主執行緒中！");
//     //     thread::sleep(Duration::from_millis(1));
//     // }

//     let handle = thread::spawn(|| {
//         for i in 1..10 {
//             println!("數字 {} 出現在產生的執行緒中！", i);
//             thread::sleep(Duration::from_millis(1));
//         }
//     });

//     handle.join().unwrap();

//     for i in 1..5 {
//         println!("數字 {} 出現在主執行緒中！", i);
//         thread::sleep(Duration::from_millis(1));
//     }

//     // handle.join().unwrap();
// }

// use std::thread;

// fn main() {
//     let v = vec![1, 2, 3];

//     let handle = thread::spawn(move || {
//         println!("這是個向量：{v:?}");
//     });

//     handle.join().unwrap();
// }

// use std::sync::mpsc;
// use std::thread;

// fn main() {
//     let (tx, rx) = mpsc::channel();

//     thread::spawn(move || {
//         let val = String::from("嗨");
//         tx.send(val).unwrap();
//     });

//     let received = rx.recv().unwrap();
//     println!("取得：{received}");
// }

// use std::sync::mpsc;
// use std::thread;
// use std::time::Duration;

// fn main() {
//     let (tx, rx) = mpsc::channel();

//     thread::spawn(move || {
//         let vals = vec![
//             String::from("執行緒"),
//             String::from("傳來"),
//             String::from("的"),
//             String::from("嗨"),
//         ];

//         for val in vals {
//             tx.send(val).unwrap();
//             thread::sleep(Duration::from_secs(1));
//         }
//     });

//     for received in rx {
//         println!("取得：{}", received);
//     }
// }

// use std::sync::mpsc;
// use std::thread;
// use std::time::Duration;

// fn main() {
//     let (tx, rx) = mpsc::channel();

//     let tx1 = tx.clone();
//     thread::spawn(move || {
//         let vals = vec![
//             String::from("執行緒"),
//             String::from("傳來"),
//             String::from("的"),
//             String::from("嗨"),
//         ];

//         for val in vals {
//             tx1.send(val).unwrap();
//             thread::sleep(Duration::from_secs(1));
//         }
//     });

//     thread::spawn(move || {
//         let vals = vec![
//             String::from("更多"),
//             String::from("給你"),
//             String::from("的"),
//             String::from("訊息"),
//         ];

//         for val in vals {
//             tx.send(val).unwrap();
//             thread::sleep(Duration::from_secs(1));
//         }
//     });

//     for received in rx {
//         println!("取得：{received}");
//     }
// }

// use std::sync::Mutex;

// fn main() {
//     let m = Mutex::new(5);

//     {
//         let mut num = m.lock().unwrap();
//         *num = 6;
//     }

//     println!("m = {m:?}");
// }

// use std::sync::{Arc, Mutex};
// use std::thread;

// fn main() {
//     let counter = Arc::new(Mutex::new(0));
//     let mut handles = vec![];

//     for _ in 0..10 {
//         let counter = Arc::clone(&counter);
//         let handle = thread::spawn(move || {
//             let mut num = counter.lock().unwrap();

//             *num += 1;
//         });
//         handles.push(handle);
//     }

//     for handle in handles {
//         handle.join().unwrap();
//     }

//     println!("結果：{}", *counter.lock().unwrap());
// }

// 死結示範
// use std::sync::{Arc, Mutex};
// use std::thread;
// use std::time::Duration;

// fn main() {
//     let lock_a = Arc::new(Mutex::new(()));
//     let lock_b = Arc::new(Mutex::new(()));

//     let a1 = Arc::clone(&lock_a);
//     let b1 = Arc::clone(&lock_b);

//     let t1 = thread::spawn(move || {
//         let _a = a1.lock().unwrap();
//         println!("執行緒 1 鎖住 A");

//         // 模擬運算延遲
//         thread::sleep(Duration::from_millis(100));

//         let _b = b1.lock().unwrap();
//         println!("執行緒 1 鎖住 B");
//     });

//     let a2 = Arc::clone(&lock_a);
//     let b2 = Arc::clone(&lock_b);

//     let t2 = thread::spawn(move || {
//         let _b = b2.lock().unwrap();
//         println!("執行緒 2 鎖住 B");

//         // 模擬運算延遲
//         thread::sleep(Duration::from_millis(100));

//         let _a = a2.lock().unwrap();
//         println!("執行緒 2 鎖住 A");
//     });

//     t1.join().unwrap();
//     t2.join().unwrap();
// }

// 避免死結
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

fn main() {
    let a = Arc::new(Mutex::new(()));
    let b = Arc::new(Mutex::new(()));

    let a1 = Arc::clone(&a);
    let b1 = Arc::clone(&b);

    let t = thread::spawn(move || {
        let _a = a1.lock().unwrap();
        thread::sleep(Duration::from_millis(100));
        if let Ok(_b) = b1.try_lock() {
            println!("成功鎖住兩者");
        } else {
            println!("避免死結：取得 B 失敗");
        }
    });

    let _b = b.lock().unwrap();
    thread::sleep(Duration::from_millis(100));
    let _a = a.lock().unwrap();

    t.join().unwrap();
}
