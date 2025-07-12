# 🔍 Rust Grep

A simple `grep`-like command-line utility built in Rust. It allows you to search for regular expression (regex) patterns in a text file and highlights the matches in red using terminal colors.

This is an educational tool that demonstrates how to build interactive CLI applications in Rust with colored output, error handling, and pattern matching.

---

## 🚀 Features

- Interactive `CMD>` prompt for entering search commands
- Accepts regex patterns as input
- Highlights all matches in red using ANSI color codes
- Graceful error handling for:
  - Invalid regex patterns
  - Missing or incorrect file paths
- Uses the powerful [`regex`](https://docs.rs/regex/) and [`termcolor`](https://docs.rs/termcolor/) crates

---

## 📦 Dependencies

Make sure your `Cargo.toml` includes the following:

```toml
[dependencies]
regex = "1.10.5"
termcolor = "1.4.1"
```

---

## ⚙️ Setup Instructions

### 1. Clone the Repository

```bash
git clone https://github.com/YOUR_USERNAME/rust-grep.git
cd rust-grep
```

### 2. (Optional) Add a Sample Text File

To test the grep tool, create a `dump.txt` file in the project root:

```bash
echo "Alice likes apples." > dump.txt
echo "Bob likes bananas." >> dump.txt
echo "Carol likes cherries." >> dump.txt
```

Make sure the file is located in the **root directory** (where `Cargo.toml` is), not in the `src/` folder.

---

## ▶️ Running the Program

To build and run the tool:

```bash
cargo run
```

You’ll see an interactive prompt:

```text
CMD>
```

Type your command in the following format:

```text
mygrep <regex-pattern> <path-to-file>
```

### 🔍 Example:

```text
mygrep apples dump.txt
```

This will highlight all matches of "apples" in red.

### ❌ To Exit:

```text
exit
```

---

## 📌 Notes

- File paths must be valid and accessible from the project root.
- `~` (home directory shortcut) is **not** expanded automatically.
  - ✅ Use `/Users/yourname/Documents/file.txt`
  - ❌ Avoid `~/Documents/file.txt`

---

## 📂 Project Structure

```
rust-grep/
├── Cargo.toml
├── src/
│   ├── main.rs
│   └── grep.rs
├── dump.txt        # Optional test file
└── README.md
```

---

## 🦀 Built With

- [Rust](https://www.rust-lang.org/)
- [regex](https://docs.rs/regex/)
- [termcolor](https://docs.rs/termcolor/)
