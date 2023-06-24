#![deny(missing_docs)]
#![forbid(unsafe_code)]

//! pathtrim - When all you need is the last few parts of a path.
//!
//! This crate implements the TrimmablePath trait on anything that implements
//! AsRef<std::path::Path> so you can easily obtain the last *n* parts of a path.
//! One good implementor that comes to mind is std::path::Path
//!
//! # Usage
//!
//! ```
//!    use std::path::Path;
//!    // at the top of your source file
//!    use pathtrim::TrimmablePath;
//!
//!    // TrimmablePath is automatically implemented for all std::path::Paths in scope
//!    let path = Path::new("/usr/local/bin/");
//!
//!    let trimmed = path.trim_to_nth(1).unwrap();
//!    assert_eq!(trimmed.to_str().unwrap(), "bin");
//!
//!    let trimmed = path.trim_to_nth(2).unwrap();
//!    assert_eq!(trimmed.to_str().unwrap(), "local/bin");
//!
//!    let trimmed = path.trim_to_nth(3).unwrap();
//!    assert_eq!(trimmed.to_str().unwrap(), "usr/local/bin");
//!
//!    let trimmed = path.trim_to_nth(4);
//!    assert!(trimmed.is_none());
//!
//!    let trimmed = path.trim_to_nth(300);
//!    assert!(trimmed.is_none());
//!
//!    let trimmed = path.trim_to_nth(2000);
//!    assert!(trimmed.is_none());
//!    
//! ```

use std::path::{Component, Path, PathBuf, MAIN_SEPARATOR};

/// The TrimmablePath trait on std::path::Path so you can easily obtain the
/// last *n* parts of anything that implements AsRef<Path>.
pub trait TrimmablePath: AsRef<Path> {
    /// Returns an Option<&Path>.
    /// If *n* is longer than the length of the Path, returns None
    ///
    /// Algorithm inspired by @nnethercote in the Zulip Rust channel:
    ///
    /// ![image](https://user-images.githubusercontent.com/24578097/145341121-1e858f4b-5ab9-436c-bcc4-9ee6effa6340.png)
    fn trim_to_nth(&self, n: usize) -> Option<PathBuf> {
        let path = self.as_ref();
        let components: Vec<_> = path.components().collect();
        let len = components.len();

        if len > n {
            let trimmed_components: Vec<_> = components.into_iter().skip(len - n).collect();
            let mut trimmed = trimmed_components.iter().collect::<PathBuf>();
            if trimmed_components.last().map(|c| c.as_os_str()) == Some(Component::RootDir.as_ref())
            {
                trimmed = trimmed.join(MAIN_SEPARATOR.to_string());
            }
            Some(trimmed)
        } else {
            None
        }
    }
}
// automagically implement for all Paths in usage scope
impl TrimmablePath for Path {}

#[cfg(test)]
mod tests {
    use std::path::Path;

    use crate::TrimmablePath;

    #[cfg(not(windows))]
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

    #[cfg(windows)]
    #[test]
    fn it_works_on_windows() {
        let p = Path::new(r"C:\Program Files\package\bin\");
        let trimmed = p.trim_to_nth(1).unwrap();
        assert_eq!(trimmed.to_str().unwrap(), "bin");
        let trimmed = p.trim_to_nth(2).unwrap();
        assert_eq!(trimmed.to_str().unwrap(), r"package\bin");
        let trimmed = p.trim_to_nth(3).unwrap();
        assert_eq!(trimmed.to_str().unwrap(), r"Program Files\package\bin");
        let trimmed = p.trim_to_nth(5); // Changed from `4` to `5`.
        assert!(trimmed.is_none());
    }
}
