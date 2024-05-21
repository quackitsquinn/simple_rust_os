use std::process::{Command, Stdio};

fn log(txt: &str) {
    println!("cargo::warning={}", txt);
}

fn main() {
    //println!("cargo::rerun-if-changed=link.ld");
    println!("cargo::rustc-link-arg=-n"); // the other project uses -n, so i will too
    println!("cargo::rustc-link-arg=-Tlink.ld");
}
