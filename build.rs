// https://github.com/grantshandy/htmxchat/blob/main/build.rs

use std::{env, process::Command};

fn main() {
    println!("cargo:rerun-if-changed=./src/main.rs");

    let tailwind_cmd = format!(
        "tailwindcss --minify -i src/input.css -o {}/output.css",
        env::var("OUT_DIR").unwrap()
    );

    let output = format!("{}/output.css", std::env::var("OUT_DIR").unwrap());

    Command::new("tailwindcss")
        .arg("--minify")
        .args(["-i", "src/input.css"])
        .args(["-o", output.as_str()])
        .status()
        .expect("error running tailwind");

    Command::new("touch").arg("women").status().expect("err");
}
