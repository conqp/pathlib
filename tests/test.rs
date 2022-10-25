use pathlib::Path;

#[test]
fn test_parts() {
    let paths: [(Vec<&str>, Path); 2] = [
        (vec!["/", "usr", "bin", "rustc"], Path::new("/usr/bin/rustc")),
        (vec!["c:/", "Program Files", "PSF"], Path::new("c:/Program Files/PSF")),
    ];

    for (nominal, path) in paths.iter() {
        assert_eq!(*nominal, path.parts())
    }
}