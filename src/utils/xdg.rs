use std::env;
use std::path::PathBuf;
use std::fs;
use directories::BaseDirs;

// get user's shell, else return '/bin/sh'
pub fn get_shell() -> PathBuf {
    let shell_bin;
    match env::var("SHELL") {
        Ok(val) => shell_bin = PathBuf::from(val),
        Err(_) => shell_bin = PathBuf::from("/bin/sh"),
    }
    shell_bin
}

pub fn get_repo_dir() -> PathBuf {
    let base_dirs = BaseDirs::new().expect("put meaningful error msg here");

    let mut local_path = base_dirs.data_local_dir().to_path_buf();
    local_path.push("luxon");
    if !local_path.exists() {
        println!("path doesn't exist, making folder");
        fs::create_dir_all(&local_path).expect("put meaningful error msg here");
    }
    local_path
}
