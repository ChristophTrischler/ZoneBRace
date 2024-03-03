use std::process::Command;

fn main() {
    println!("cargo:rerun-if-changed=templates/");
    //println!("cargo:rerun-if-changed=tailwind.config.js");
    if let Err(e) = Command::new("tailwindcss")
        .args(&["-o", "static/style.css"])
        .status()
    {
        eprintln!("{e:?}");
    }
}
