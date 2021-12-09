#![deny(missing_docs)]
#![forbid(unsafe_code)]

//! pathtrim - When all you need is the last few parts of a path.
//!
//! This crate implements the TrimmablePath trait on std::path::Path so you can easily obtain the
//! last *n* parts of a path.
//!
//! # Usage
//!
//! ```text
//! // in Cargo.toml
//! [dependencies]
//! pathtrim = "1.0.0"
//! ```
//!
//! ```
//!    # use std::path::Path;
//!    // at the top of your source file
//!    use pathtrim::TrimmablePath;
//!
//!    // TrimmablePath is automatically implemented for all std::path::Paths in scope
//!    let path = Path::new("/usr/local/bin/");
//!    let trimmed = path.trim_to_nth(2);
//!
//!    assert!(trimmed.is_some());
//!    assert_eq!(trimmed.unwrap().to_str().unwrap(), "local/bin");
//!
//!    let trimmed = path.trim_to_nth(2000);
//!    assert!(trimmed.is_none());
//!    
//! ```

use std::path::Path;

/// The TrimmablePath trait on std::path::Path so you can easily obtain the
/// last *n* parts of a path.
pub trait TrimmablePath: AsRef<Path> {
    /// Returns an Option<&Path> in case *n* is longer
    /// than the length of the Path, otherwise None
    /// Algorithm inspired by @nnethercote in the Zulip Rust channel: ![image](https://user-images.githubusercontent.com/24578097/145341121-1e858f4b-5ab9-436c-bcc4-9ee6effa6340.png)
    fn trim_to_nth(&self, n: usize) -> Option<&Path> {
        let path = self.as_ref();
        let len = path.components().count();
        if len > n {
            let mut c = path.components();
            c.nth(len - (n + 1));
            Some(c.as_path())
        } else {
            None
        }
    }
}

impl TrimmablePath for Path {}

#[cfg(test)]
mod tests {
    use std::path::Path;

    use crate::TrimmablePath;

    #[test]
    fn it_works() {
        let p = Path::new("/usr/local/bin/");
        let trimmed = p.trim_to_nth(1).unwrap();
        assert_eq!(trimmed.to_str().unwrap(), "bin");
        let trimmed = p.trim_to_nth(2).unwrap();
        assert_eq!(trimmed.to_str().unwrap(), "local/bin");
        let trimmed = p.trim_to_nth(3).unwrap();
        assert_eq!(trimmed.to_str().unwrap(), "usr/local/bin");
        let trimmed = p.trim_to_nth(300);
        assert!(trimmed.is_none());
    }
}
