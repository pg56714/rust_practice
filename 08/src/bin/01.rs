use std::collections::HashMap;

fn median_and_mode(nums: &mut Vec<i32>) -> (f64, Vec<i32>) {
    // 1. 對數列排序
    nums.sort();
    println!("排序後的 nums: {:?}", nums);

    // 2. 計算中位數
    let median = if nums.len() % 2 == 0 {
        let mid = nums.len() / 2;
        println!("偶數長度，兩個中間值：{} 和 {}", nums[mid - 1], nums[mid]);
        (nums[mid - 1] + nums[mid]) as f64 / 2.0
    } else {
        let mid = nums.len() / 2;
        println!("奇數長度，中間值：{}", nums[mid]);
        nums[mid] as f64
    };
    println!("中位數為: {}", median);

    // 3. 使用 HashMap 計算每個數字出現的次數
    let mut counts = HashMap::new();
    for &num in nums.iter() {
        *counts.entry(num).or_insert(0) += 1;
    }
    println!("每個數字的出現次數: {:?}", counts);

    // 4. 找出最大出現次數
    let max_count = counts.values().cloned().max().unwrap_or(0);
    println!("最大出現次數: {}", max_count);

    // 5. 過濾出所有達到最大出現次數的數字（可能有多個）
    let modes = counts
        .into_iter()
        .filter(|&(_, count)| count == max_count)
        .map(|(num, _)| num)
        .collect();
    println!("眾數為: {:?}", modes);

    (median, modes)
}

fn main() {
    let mut nums = vec![1, 2, 2, 3, 4];
    println!("原始數列: {:?}", nums);

    let (median, modes) = median_and_mode(&mut nums);
    println!("最終結果：");
    println!("中位數: {}", median);
    println!("眾數: {:?}", modes);
}
