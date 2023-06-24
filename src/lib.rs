#![forbid(unsafe_code)]

//! pathtrim - When all you need is the last few parts of a path.
//!
//! This crate implements the TrimmablePath trait on anything that implements
//! AsRef<std::path::Path> so you can easily obtain the last *n* parts of a path.
//! One good implementor that comes to mind is std::path::Path
//!
//! # Usage
//!

use std::collections::VecDeque;
use std::path::{Path, PathBuf};

pub trait TrimmablePath: AsRef<Path> {
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
        let trimmed = p.trim_to_nth(1);
        assert!(trimmed.is_none());
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
        assert_eq!(trimmed.to_str().unwrap(), "Library/Application Support/package");
        let trimmed = p.trim_to_nth(4);
        assert!(trimmed.is_none());
    }
}
