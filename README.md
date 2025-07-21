# rust_practice

English | [繁體中文](./README_zh.md)

## 🦀 Rust Development Workflow

1️⃣ `cargo fmt`  
→ Format the code to ensure consistent style.

2️⃣ `cargo check`  
→ Quickly check for syntax and type errors without building the executable.

3️⃣ `cargo clippy`  
→ Advanced linter for best practices and detecting potential issues.

> Clippy includes all checks from `cargo check` plus additional lints.

4️⃣ `cargo build`  
→ Actually compile the code and generate the executable.  
  ⤷ If there are errors, use `cargo fix` to automatically fix what’s fixable.

5️⃣ `cargo run`  
→ Build (if necessary) and run the program.  
**Tip:** Equivalent to running `cargo build` followed by executing the binary—so if your goal is to test the program, you can skip `cargo build` and just use `cargo run`.

6️⃣ `cargo test` _(optional)_  
→ Run unit and integration tests.  
Includes build (via `test`) and executes tests—recommended to add in CI.

🏁 **🍒 Bonus:** `cargo build --release`  
→ Compile the code using the **release** profile, enabling optimizations to generate a **smaller and faster** executable.

```bash
cargo build --release
cargo run --release
```

🚀 Publishing to crates.io  
7️⃣ cargo publish --dry-run  
→ Simulate packaging and verify metadata, dependencies, compile-check, etc.

8️⃣ cargo publish  
→ Uploads to crates.io—publishing is permanent (you cannot overwrite a version)
