# pathtrim
## When all you need is the last few parts of a path.

This crate implements the TrimmablePath trait on anything that implements AsRef<[std::path::Path](https://doc.rust-lang.org/std/path/struct.Path.html)> so you can easily obtain the last *n* parts of a path.  One good implementor of AsRef<[std::path::Path](https://doc.rust-lang.org/std/path/struct.Path.html)> that comes to mind is [std::path::Path](https://doc.rust-lang.org/std/path/struct.Path.html) 

# Usage

```rust
   use std::path::Path;
   // at the top of your source file
   use pathtrim::TrimmablePath;

   // TrimmablePath is automatically implemented for all 
   // std::path::Paths in the module's scope

   let path = Path::new("/usr/local/bin/");
   
   let trimmed = path.trim_to_nth(1).unwrap();
   assert_eq!(trimmed.to_str().unwrap(), "bin");
   
   let trimmed = path.trim_to_nth(2).unwrap();
   assert_eq!(trimmed.to_str().unwrap(), "local/bin");
   
   let trimmed = path.trim_to_nth(3).unwrap();
   assert_eq!(trimmed.to_str().unwrap(), "usr/local/bin");
   
   let trimmed = path.trim_to_nth(4);
   assert!(trimmed.is_none());
   
   let trimmed = path.trim_to_nth(300);
   assert!(trimmed.is_none());
   
   let trimmed = path.trim_to_nth(2000);
   assert!(trimmed.is_none());
  
```
*note: patch version bumps since 1.0.0 are just documentation updates*
