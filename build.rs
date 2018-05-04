use std::env;
use std::fs;
use std::path::Path;

fn main() {

    let home_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    let home_dir = Path::new(&home_dir);

    let profile = env::var("PROFILE").unwrap();
    let target_dir = home_dir.join("target").join(profile);

    copy_all(&home_dir.join("assets"), &target_dir.join("assets"));

    return
}

fn copy_all(source: &Path, target: &Path) {
    if let Ok(entries) = fs::read_dir(source) {
        for entry in entries {
            if let Ok(entry) = entry {

                if entry.path().is_dir() {
                    fs::create_dir_all(&target.join(entry.file_name()));
                    copy_all(&source.join(entry.file_name()), &target.join(entry.file_name()));
                }

                fs::copy(source.join(entry.file_name()), target.join(entry.file_name()));
                println!("{:?}", source.join(entry.file_name()));
            }
        }
    }
}