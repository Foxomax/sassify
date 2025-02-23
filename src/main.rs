mod figlet_module;
mod compile_module;

use std::path::Path;
use std::process::{Command, Output, Stdio};
use std::fs;
use std::io::{self, Write};

fn main() -> io::Result<()> {
    // Variables
    let mut child: Output;
    let mut sass_main_file = String::new();
    let figlet = figlet_module::FigletModule::new("./bin/standard.flf");

    // Welcome to the program
    Command::new("clear")
        .spawn()
        .expect("Failed to execute command");

    figlet.run("Sass Compiler");

    child = Command::new("pwd")
        .output()
        .expect("Failed to execute command");

    // Obtain the path from the output of the previous command
    let path = String::from_utf8_lossy(&child.stdout).trim().to_string();
    println!("Directorio actual: {}", path);
    sass_main_file = get_sass_main_file(path)?;
    println!("Archivo principal de Sass: {}", sass_main_file);

    Ok(())
}

fn get_config_file<P: AsRef<Path>>(dir: P) -> io::Result<String> {
    let entries = fs::read_dir(&dir)?;

    for entry in entries {
        let entry = entry?;
        let file_name = entry.file_name();
        let file_name_str = file_name.to_string_lossy();
        if file_name_str.contains("config") && file_name_str.contains("json") {
            return Ok(file_name_str.to_string());
        }
    }

    eprintln!("Error: No se encontró ningún archivo de configuración en {:?}", dir.as_ref());
    print!("Por favor, ingresa el nombre del archivo manualmente: ");
    io::stdout().flush()?;

    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    Ok(input.trim().to_string())
}