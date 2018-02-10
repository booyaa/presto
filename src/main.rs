extern crate magic;

use magic::{Cookie, CookieFlags};

fn main() {
    // Create a new default configuration
    let cookie = Cookie::open(CookieFlags::default()).unwrap();

    // wtf? found in https://github.com/jkcclemens/bins/blob/42caed1d6f7a34dffd38e75b1f0ec32c7d44c87e/src/main.rs#L524
    // cookie.load(&[""; 0]).unwrap();

    let databases = vec!["/usr/local/Cellar/libmagic/5.32/share/misc/magic"];
    assert!(cookie.load(&databases).is_ok());

    let files = [
        "Cargo.toml",
        ".vscode/settings.json",
        "target/debug/presto",
        "../../rust/learn-intermezzos/build/boot.o",
        "../../rust/learn-intermezzos/build/os.iso",
    ];

    // let file = "Cargo.toml";                                     // ASCII text
    // let file = ".vscode/settings.json";                          // ASCII text
    // let file = "target/debug/presto";                            // Mach-O 64-bit x86_64 executable, flags:<NOUNDEFS|DYLDLINK|TWOLEVEL|WEAK_DEFINES|BINDS_TO_WEAK|PIE|HAS_TLV_DESCRIPTORS>
    // let file = "../../rust/learn-intermezzos/build/boot.o";      // ELF 64-bit LSB relocatable, x86-64, version 1 (SYSV), not stripped
    // let file = "../../rust/learn-intermezzos/build/kernel.bin";  // ELF 64-bit LSB executable, x86-64, version 1 (SYSV), statically linked, not stripped
    // let file = "../../rust/learn-intermezzos/build/os.iso";         // DOS/MBR boot sector; GRand Unified Bootloader, stage1 version 0x79, boot drive 0xbb, stage2 address 0x8e70, 1st sector stage2 0xb8db31c3, stage2 segment 0x201
    // println!("{}: {}", file, cookie.file(&file).unwrap());
    for file in files.iter() {
        println!("{}: {}", file, cookie.file(&file).unwrap());
    }

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

    println!("{:?}", &[""; 0]);
}
