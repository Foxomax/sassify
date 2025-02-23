use std::{fs, io};
use std::process::Command;
use std::path::Path;

pub struct FigletModule {
    pub font: String,
}

impl FigletModule {
    pub fn new(font: &str) -> Self {
        FigletModule {
            font: font.to_string(),
        }
    }

    pub fn run(&self, text: &str) {
        let figlet_path = Path::new("./bin/figlet");

        let output = Command::new(figlet_path)
            .arg("-f")
            .arg(&self.font)
            .arg(text) // Usamos `text` aquí
            .output()
            .expect("Failed to execute figlet command");

        if !output.status.success() {
            eprintln!("Error: {}", String::from_utf8_lossy(&output.stderr));
            return;
        }

        println!("{}", String::from_utf8_lossy(&output.stdout));
    }
}