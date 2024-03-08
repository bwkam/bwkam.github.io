use std::process::Command;

fn main() {
    // TODO: will probably need to add more paths
    println!("cargo:rerun-if-changed=./src");

    // let output = format!("{}/output.css", std::env::var("OUT_DIR").unwrap());
    let output = format!("{}/output.css", "assets/css");

    Command::new("tailwindcss")
        .arg("--minify")
        .args(["-i", "src/input.css"])
        .args(["-o", output.as_str()])
        .status()
        .expect("error running tailwind");
    
    // Command::new("touch").arg("something").status().expect("err");
}
