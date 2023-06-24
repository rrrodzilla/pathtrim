# pathtrim

`pathtrim` is a simple, yet powerful Rust library that helps you obtain the last *n* parts of a filesystem path. It provides a seamless way to work with paths in a cross-platform manner, making it a great choice for projects that need to run on different systems like Unix, macOS, and Windows.

With `pathtrim`, you get a clean and intuitive API for handling path trimming, out-of-the-box support for different platforms, and the confidence that comes with strong safety guarantees, thanks to the use of Rust's features and ecosystem.

[![Crates.io](https://img.shields.io/crates/v/pathtrim)](https://crates.io/crates/pathtrim)
[![Crates.io (Downloads)](https://img.shields.io/crates/d/pathtrim)](https://crates.io/crates/pathtrim)
[![Continuous integration](https://github.com/rrrodzilla/pathtrim/actions/workflows/Continuous%20Integration.yml/badge.svg)](https://github.com/rrrodzilla/pathtrim/actions/workflows/Continuous%20Integration.yml)
[![Docs.rs](https://docs.rs/pathtrim/badge.svg)](https://docs.rs/pathtrim)

## 💡 Features

- Obtain the last *n* components of a path with just a line of code.
- Cross-platform: works seamlessly on Unix, macOS, and Windows.
- Built upon the powerful [std::path](https://doc.rust-lang.org/stable/std/path/index.html) module.
- Highly configurable, with a simple and easy-to-understand API.

## 🚀 Getting Started

Add this to your `Cargo.toml`:

```toml
[dependencies]
pathtrim = "0.2.0"
```

And then import the trait in the file where you want to use it:

```rust
use std::path::Path;
use pathtrim::TrimmablePath;
```

### Examples

#### Basic Usage

```rust
use std::path::Path;
use pathtrim::TrimmablePath;

let path = Path::new("/usr/local/bin/application");
let trimmed = path.trim_to_nth(2).unwrap();
assert_eq!(trimmed.to_str().unwrap(), "bin/application");
```

#### Different Platforms

Unix:

```rust
use std::path::Path;
use pathtrim::TrimmablePath;
let path = Path::new("/usr/local/bin/application");
let trimmed = path.trim_to_nth(3).unwrap();
assert_eq!(trimmed.to_str().unwrap(), "local/bin/application");
```

Windows:

```rust
use std::path::Path;
use pathtrim::TrimmablePath;
let path = Path::new(r"C:\Program Files\package\bin\application");
let trimmed = path.trim_to_nth(2).unwrap();
assert_eq!(trimmed.to_str().unwrap(), r"bin\application");
```

#### Edge Cases

```rust
use std::path::Path;
use pathtrim::TrimmablePath;
let path = Path::new("/");
let trimmed = path.trim_to_nth(1);
assert!(trimmed.is_none());
```

## 📚 Further Reading

To gain a deeper understanding of path components and various related APIs, refer to the official Rust documentation:

- [std::path](https://doc.rust-lang.org/stable/std/path/index.html)
- [std::path::Components](https://doc.rust-lang.org/stable/std/path/struct.Components.html)

## License

This project is licensed under the [MIT License](LICENSE).
