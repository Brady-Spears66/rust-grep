# 🔍 Rust Grep

A simple `grep`-like command-line utility built in Rust. It allows you to search for regex patterns in a text file and highlights the matches in red using terminal colors.

---

## 🚀 Features

- Accepts regex patterns as input
- Highlights matches in red
- Interactive `CMD>` prompt interface
- Graceful error handling for invalid files and patterns
- Uses [`regex`](https://docs.rs/regex/) and [`termcolor`](https://docs.rs/termcolor/) crates

---

## 📦 Dependencies

Make sure your `Cargo.toml` includes:

```toml
[dependencies]
regex = "1.10.5"
termcolor = "1.4.1"
```
