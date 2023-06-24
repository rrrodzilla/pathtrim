# Changelog

## 2023-06-23

### Added
- Add cross-platform support for `TrimmablePath` trait (#4ced091, #19e8000)
- Add GitHub Actions configuration for Continuous Integration (#9613cd1)
- Add comprehensive tests for `TrimmablePath` trait (#39eab80)

### Changed
- Optimize `trim_to_nth` function with single iteration (#9b6a12b)
- Update documentation to explain `AsRef<Path>` (#ea036ca)
- Update readme in preparation for 2.0 launch (#1ee5724)

### Fixed
- Resolve Cargo.toml lock issue in Continuous Integration.yml (#213108c)
- Make sure Linux tests do not run on Windows (#8896cfd)
- Update unit test for Windows (#c44ab1f)
- Update tests and workflow (#bba95c1)
- Remove doctest for now (#4a96fe4)
- Correct Windows edge case test in `trimmable_path` (#b6041bb)
- Add missing Rust documentation comments for `TrimmablePath` trait (#393b679)
- Prevent doctests from running on Windows (#0221e31)

### Maintenance
- Initial commit (#bc9c13b)
- Add readme and finalize Cargo.toml for publishing (#6fda26c)
- Fix typos in the crate name (#2f7160a)
- Fix typo in readme and bump patch ver to republish (#2713099)
- Bump patch version and fix documentation formatting (#92b7c52, #1de2349)
