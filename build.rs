use std::env;
use std::fs;
use std::path::{Path, PathBuf};

const COPY_DIR: &'static str = "assets";

fn copy_dir<P, Q>(from: P, to: Q)
where
    P: AsRef<Path>,
    Q: AsRef<Path>,
{
    let to = to.as_ref().to_path_buf();

    for path in fs::read_dir(from).unwrap() {
        let path = path.unwrap().path();
        let to = to.clone().join(path.file_name().unwrap());

        if path.is_file() {
            fs::copy(&path, to).unwrap();
        } else if path.is_dir() {
            if !to.exists() {
                fs::create_dir(&to).unwrap();
            }

            copy_dir(&path, to);
        }
    }
}

fn main() {
    let out = env::var("PROFILE").unwrap();
    let out = PathBuf::from(format!("target/{}/{}", out, COPY_DIR));

    if out.exists() {
        fs::remove_dir_all(&out).unwrap();
    }

    fs::create_dir(&out).unwrap();
    copy_dir(COPY_DIR, &out);
}
