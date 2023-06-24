#![forbid(unsafe_code)]
#![deny(missing_docs)]
#![deny(warnings)]
#![deny(missing_debug_implementations)]
#![deny(unused_imports)]

//! pathtrim - When all you need is the last few parts of a path.
//!
//! This crate provides a simple way to obtain the last *n* parts of a path by implementing the TrimmablePath trait on anything that implements AsRef<std::path::Path>.
//!
//! # Edge Cases & Limitations
//!
//! - The crate supports both Unix and Windows paths.
//! - It doesn't handle cases where the path is double-escaped or contains invalid components.
//! - When using trim_to_nth, the number of parts to be trimmed should be less than the total number of components in the path, otherwise it returns None.
//!
//! # Usage
//!
//! Add this to your `Cargo.toml`:
//!
//! ```toml
//! [dependencies]
//! pathtrim = "0.2.0"
//! ```
//!
//! And this to your main file:
//!
//! ```rust
//! use std::path::Path;
//! use pathtrim::TrimmablePath;
//! ```
//!
//! # Examples
//!
//! ## Basic usage
//!
//! ```no_run
//! use std::path::Path;
//! use pathtrim::TrimmablePath;
//!
//! let path = Path::new("/usr/local/bin/application");
//! let trimmed = path.trim_to_nth(2).unwrap();
//! assert_eq!(trimmed.to_str().unwrap(), "bin/application");
//! ```
//!
//! ## Different platforms
//!
//! Unix:
//!
//! ```no_run
//! use std::path::Path;
//! use pathtrim::TrimmablePath;
//! let path = Path::new("/usr/local/bin/application");
//! let trimmed = path.trim_to_nth(3).unwrap();
//! assert_eq!(trimmed.to_str().unwrap(), "local/bin/application");
//! ```
//!
//! Windows:
//!
//! ```no_run
//! use std::path::Path;
//! use pathtrim::TrimmablePath;
//! let path = Path::new(r"C:\Program Files\package\bin\application");
//! let trimmed = path.trim_to_nth(2).unwrap();
//! assert_eq!(trimmed.to_str().unwrap(), r"bin\application");
//! ```
//!
//! ## Edge Cases
//!
//! ```no_run
//! use std::path::Path;
//! use pathtrim::TrimmablePath;
//! let path = Path::new("/");
//! let trimmed = path.trim_to_nth(1);
//! assert!(trimmed.is_none());
//! ```
//!
//! # Further Reading
//!
//! To understand further about path components and various related APIs, refer to the official Rust documentation:
//!
//! - [std::path](https://doc.rust-lang.org/stable/std/path/index.html)
//! - [std::path::Components](https://doc.rust-lang.org/stable/std/path/struct.Components.html)
use std::collections::VecDeque;
use std::path::{Path, PathBuf};
/// A trait to easily obtain the last *n* parts of a path.
///
/// This trait is implemented for anything that implements
/// `AsRef<std::path::Path>`, so you can use it on types like
/// `std::path::Path` and `std::path::PathBuf`.
///
/// # Examples
///
/// ```no_run
/// use std::path::Path;
/// use pathtrim::TrimmablePath;
///
/// let p = Path::new("/usr/local/bin/");
/// let trimmed = p.trim_to_nth(1).unwrap();
/// assert_eq!(trimmed.to_str().unwrap(), "bin");
/// ```
pub trait TrimmablePath: AsRef<Path> {
    /// Returns a new `PathBuf` containing the last `n` components of the path,
    /// or `None` if there are not enough components in the path.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use std::path::Path;
    /// use pathtrim::TrimmablePath;
    ///
    /// let p = Path::new("/usr/local/bin/");
    /// let trimmed = p.trim_to_nth(2).unwrap();
    /// assert_eq!(trimmed.to_str().unwrap(), "local/bin");
    /// ```
    fn trim_to_nth(&self, n: usize) -> Option<PathBuf> {
        let path = self.as_ref();
        let components_iter = path.components();

        if n >= path.components().count() {
            return None;
        }

        let mut last_n_components = VecDeque::with_capacity(n);

        for component in components_iter {
            println!("component: {:?}", component);
            println!("n: {n}");
            println!("last_n_components.len(): {:?}", last_n_components.len());

            if last_n_components.len() == n {
                println!("bm 1");
                last_n_components.pop_front();
            }
            println!("bm 2");
            last_n_components.push_back(component);
        }

        if last_n_components.len() > n {
            println!("bm 3");
            None
        } else {
            println!("bm 4");
            let mut trimmed = PathBuf::new();
            println!("bm 5");
            for component in last_n_components {
                trimmed.push(component);
            }
            println!("bm 7");
            println!("trimmed {:?}", trimmed);
            Some(trimmed)
        }
    }
} // automagically implement for all Paths in usage scope
impl TrimmablePath for Path {}

#[cfg(test)]
mod tests {
    use std::path::Path;

    use crate::TrimmablePath;

    #[cfg(not(windows))]
    #[test]
    fn trimmable_path_unix_basic_cases() {
        let p = Path::new("/usr/local/bin/");
        let trimmed = p.trim_to_nth(1).unwrap();
        assert_eq!(trimmed.to_str().unwrap(), "bin");
        let trimmed = p.trim_to_nth(2).unwrap();
        assert_eq!(trimmed.to_str().unwrap(), "local/bin");
        let trimmed = p.trim_to_nth(3).unwrap();
        assert_eq!(trimmed.to_str().unwrap(), "usr/local/bin");
        let trimmed = p.trim_to_nth(4);
        assert!(trimmed.is_none());
    }

    #[cfg(not(windows))]
    #[test]
    fn trimmable_path_unix_edge_cases() {
        let p = Path::new("/");
        let trimmed = p.trim_to_nth(1);
        assert!(trimmed.is_none());
    }

    #[cfg(windows)]
    #[test]
    fn trimmable_path_windows_basic_cases() {
        let p = Path::new(r"C:\Program Files\package\bin\");
        let trimmed = p.trim_to_nth(1).unwrap();
        assert_eq!(trimmed.to_str().unwrap(), "bin");
        let trimmed = p.trim_to_nth(2).unwrap();
        assert_eq!(trimmed.to_str().unwrap(), r"package\bin");
        let trimmed = p.trim_to_nth(3).unwrap();
        assert_eq!(trimmed.to_str().unwrap(), r"Program Files\package\bin");
        let trimmed = p.trim_to_nth(5);
        assert!(trimmed.is_none());
    }

    #[cfg(windows)]
    #[test]
    fn trimmable_path_windows_edge_cases() {
        let p = Path::new(r"C:\");
        let trimmed = p.trim_to_nth(1).unwrap();
        assert_eq!(trimmed.to_str().unwrap(), r"\");
    }

    #[cfg(any(target_os = "macos", target_os = "ios"))]
    #[test]
    fn trimmable_path_macos_basic_cases() {
        let p = Path::new("/Library/Application Support/package/");
        let trimmed = p.trim_to_nth(1).unwrap();
        assert_eq!(trimmed.to_str().unwrap(), "package");
        let trimmed = p.trim_to_nth(2).unwrap();
        assert_eq!(trimmed.to_str().unwrap(), "Application Support/package");
        let trimmed = p.trim_to_nth(3).unwrap();
        assert_eq!(
            trimmed.to_str().unwrap(),
            "Library/Application Support/package"
        );
        let trimmed = p.trim_to_nth(4);
        assert!(trimmed.is_none());
    }
}
