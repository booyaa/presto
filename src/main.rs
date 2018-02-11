extern crate magic;

use std::env;
use magic::{Cookie, CookieFlags};

fn demo(cookie: &Cookie) {
    let files = [
        "Cargo.toml",
        ".vscode/settings.json",
        "target/debug/presto",
        "../../rust/learn-intermezzos/build/boot.o",
        "../../rust/learn-intermezzos/build/os.iso",
    ];

    for file in files.iter() {
        println!("{}:\n\t{}", file, cookie.file(&file).unwrap());
    }
}

#[allow(dead_code)]
fn demo_vec(cookie: &Cookie) {
    let mut files_vec = vec![
        "Cargo.toml",
        ".vscode/settings.json",
        "target/debug/presto",
        "../../rust/learn-intermezzos/build/boot.o",
        "../../rust/learn-intermezzos/build/os.iso",
    ];

    for file in &files_vec {
        println!("{}: {}", file, cookie.file(&file).unwrap());
    }

    files_vec.push(".gitignore");

    for file in &files_vec {
        println!("{}: {}", file, cookie.file(&file).unwrap());
    }
}
fn main() {
    let cookie = Cookie::open(CookieFlags::default()).unwrap();
    cookie.load(&[""; 0]).unwrap(); // wtf? found in https://github.com/jkcclemens/bins/blob/42caed1d6f7a34dffd38e75b1f0ec32c7d44c87e/src/main.rs#L524

    let args: Vec<String> = env::args().collect(); // why can't i do this? let args = env::args();

    if args.len() < 2 {
        // no file passed, run demo mode.
        demo(&cookie);
    } else {
        let file = &args[1];
        println!("{}:\n\t{}", file, cookie.file(&file).unwrap());
    }
}
