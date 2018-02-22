extern crate magic;

use std::env;
use magic::{Cookie, CookieFlags};

fn demo(cookie: &Cookie) {
    let files = [
        "Cargo.toml",
        ".vscode/settings.json",
        "target/debug/presto",
        ".",
    ];

    for file in files.iter() {
        println!("{}:\n\t{}", file, cookie.file(&file).unwrap());
    }
}

#[allow(dead_code)]
fn mc_hammer(cookie: &Cookie) {
    let mut files_vec = vec![
        "Cargo.toml",
        ".vscode/settings.json",
        "target/debug/presto",
        ".",
    ];

    for file in &files_vec {
        println!("{}:\n\t{}", file, &cookie.file(&file).unwrap());
    }

    files_vec.push(".gitignore"); // <-- vec ftw!

    for file in &files_vec {
        println!("{}:\n\t{}", file, cookie.file(&file).unwrap());
    }
}
fn main() {
    let cookie = Cookie::open(CookieFlags::default()).unwrap();
    cookie.load(&[""; 0]).unwrap();
    
    let args: Vec<String> = env::args().collect(); // why can't i do this? let args = env::args();

    let demo_mode = args.len() < 2;

    if demo_mode {
        demo(&cookie);
    } else {
        let file = &args[1];
        println!("{}:\n\t{}", file, cookie.file(&file).unwrap());
    }
}
