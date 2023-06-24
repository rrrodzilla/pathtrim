use std::path::Path;
use pathtrim::TrimmablePath;

#[cfg(not(windows))]
#[test]
fn it_works_1() {
    let p = Path::new("/usr/local/bin/");
    let trimmed = p.trim_to_nth(1).unwrap();
    assert_eq!(trimmed.to_str().unwrap(), "bin");
    let trimmed = p.trim_to_nth(2).unwrap();
    assert_eq!(trimmed.to_str().unwrap(), "local/bin");
    let trimmed = p.trim_to_nth(3).unwrap();
    assert_eq!(trimmed.to_str().unwrap(), "usr/local/bin");
    let trimmed = p.trim_to_nth(4);
    assert!(trimmed.is_none());
    let trimmed = p.trim_to_nth(300);
    assert!(trimmed.is_none());
}

#[cfg(not(windows))]
#[test]
fn it_works_2() {
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


