use std::process::Command;

fn main() {
    println!("Building tailwindcss");

    Command::new("build/tailwindcss")
        .args([
            "--input",
            "src/base.css",
            "--output",
            "assets/style.css",
            "--minify",
        ])
        .status()
        .expect("Failed to create tailwind css file");
}
