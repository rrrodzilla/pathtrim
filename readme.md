# trimpath
## When all you need is the last few parts of a path.

This crate implements the TrimmablePath trait on [std::path::Path](https://doc.rust-lang.org/std/path/struct.Path.html) so you can easily obtain the
last *n* parts of a path.

# Usage

```text
// in Cargo.toml
[dependencies]
pathtrim = "1.0.0"
```

```rust
   use std::path::Path;
   // at the top of your source file
   use pathtrim::TrimmablePath;

   // TrimmablePath is automatically implemented for all std::path::Paths in scope
   let path = Path::new("/usr/local/bin/");
   let trimmed = path.trim_to_nth(2);

   assert!(trimmed.is_some());
   assert_eq!(trimmed.unwrap().to_str().unwrap(), "local/bin");

   let trimmed = path.trim_to_nth(2000);
   assert!(trimmed.is_none());
   
```

