use ch11::add;
mod common; // 匯入 tests/common/mod.rs

#[test]
fn it_adds_two() {
    common::setup(); // 使用共用初始化函式
    assert_eq!(add(2, 2), 4);
}
