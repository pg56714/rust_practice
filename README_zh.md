# rust_practice

## 🦀 Rust 開發流程

1️⃣ `cargo fmt`  
→ 格式化程式碼（保持風格一致）

2️⃣ `cargo check`  
→ 快速檢查語法與型別錯誤（不產生執行檔）

3️⃣ `cargo clippy`  
→ 進階程式碼風格檢查與潛在問題提醒（Lint 工具）

> Clippy 類似 `check`，但提供更多最佳實踐建議與細節提示。

4️⃣ `cargo build`  
→ 實際編譯程式碼並產出執行檔  
  ⤷ 如果出現錯誤，可用 `cargo fix` 自動修正可修復的部分

5️⃣ `cargo run`  
→ 編譯（如有變動）並執行程式  
→ **Tip：** 若目的是執行程式，可直接使用 `cargo run`，無需先跑 `build`

6️⃣ `cargo test` _(可選)_  
→ 執行單元測試與整合測試，包含編譯與測試過程  
→ 建議在 CI 中加入以確保程式正確性

🏁 **🍒 額外補充：** `cargo build --release`  
→ 使用 **release profile** 編譯程式，啟用最佳化，生成 **更小更快** 的可執行檔。

```bash
cargo build --release
cargo run --release
```

🚀 發佈到 crates.io  
7️⃣ cargo publish --dry-run  
→ 模擬包裝並驗證 metadata、依賴、是否可編譯等步驟

8️⃣ cargo publish  
→ 真正上傳到 crates.io，一旦發布無法覆蓋相同版本
