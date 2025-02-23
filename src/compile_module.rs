use std::process::{Command, Output};
use std::fs;
use std::io::{self, Write};
use std::path::Path;
mod figlet_module;

struct ConfigFile {
    output_type: String,
    auto_fixer: bool,
    multiple_files: bool,
    output_dir: String,
    input_dir: String,
    main_file: String,
}

struct CompileModule {
    config_file: ConfigFile,
}

impl CompileModule {
    fn new(
        output_type: &str,
        auto_fixer: bool,
        multiple_files: bool,
        output_dir: &str,
        input_dir: &str,
        main_file: &str,
    ) -> Self {
        CompileModule {
            config_file: ConfigFile {
                output_type: output_type.to_string(),
                auto_fixer,
                multiple_files,
                output_dir: output_dir.to_string(),
                input_dir: input_dir.to_string(),
                main_file: main_file.to_string(),
            },
        }
    }

    fn run(&self) {
        let figlet = figlet_module::FigletModule::new("./bin/standard.flf");

        Command::new("clear")
            .spawn()
            .expect("Failed to execute clear command");

        figlet.run("sassify compiler");

        let child: Output = Command::new("pwd")
            .output()
            .expect("Failed to execute pwd command");

        let path = String::from_utf8_lossy(&child.stdout).trim().to_string();
        println!("Directorio actual: {}", path);

        let sass_main_file = Self::get_sass_main_file(path).unwrap();
        println!("Archivo principal de Sass: {}", sass_main_file);
    }

    fn get_sass_main_file<P: AsRef<Path>>(dir: P) -> io::Result<String> {
        let entries = fs::read_dir(&dir)?;

        for entry in entries {
            let entry = entry?;
            let file_name = entry.file_name();
            let file_name_str = file_name.to_string_lossy();
            println!("Archivo encontrado: {}", file_name_str); // Debug
            if file_name_str.contains("main") && file_name_str.contains("scss") {
                return Ok(file_name_str.to_string());
            }
        }

        eprintln!(
            "Error: No se encontró ningún archivo principal de Sass en {:?}",
            dir.as_ref()
        );
        print!("Por favor, ingresa el nombre del archivo manualmente: ");
        io::stdout().flush()?;

        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        Ok(input.trim().to_string())
    }
}
