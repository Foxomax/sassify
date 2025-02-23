# Sassify

**Sassify** is a plugin for Sass written in **Rust** that supercharges your Sass compilation process by adding advanced functionalities and real-time feedback. With Sassify, you can compile multiple Sass files at once, automatically fix errors on every update, and receive immediate error reporting—all fully configurable via a dedicated configuration file.

## Features

- **Multiple Compilation:**  
  Compile several Sass files simultaneously to ensure all your stylesheets are always up-to-date.

- **Autofixer on Updates:**  
  Automatically correct common errors and formatting issues every time a Sass file is modified—no more tedious manual fixes!

- **Real-Time Error Reporting:**  
  Get instant feedback on compilation errors as they occur, allowing you to quickly identify and resolve issues without interrupting your workflow.

- **Fully Configurable:**  
  Customize every aspect of Sassify through its configuration file. Tailor the plugin to perfectly match your project’s structure and requirements.

- **Optional Live Reload:**  
  Integrate live reload support for immediate visual feedback during development, making your coding sessions even more dynamic.

## Installation

### Prerequisites

- [Rust](https://www.rust-lang.org/tools/install) (with Cargo, Rust’s package manager).

### Build from Source

Clone the repository and compile Sassify:

```bash
git clone https://github.com/your_username/sassify.git
cd sassify
cargo build --release
```

## Configuration
Customize Sassify by creating a configuration file in your project root (sassify.config.json). Below is an example configuration:
```json
{
  "output_type": "compressed",
  "auto_fixer": false,
  "multiple_files": true,
  "output_dir": ".",
  "main_file": "main.scss",
  "iput_dir": ["./sass/"]
}
```

## Credits

Sassify uses [Figlet](https://www.figlet.org/) to generate eye-catching ASCII art in the terminal, adding a unique visual touch to the tool. We would like to thank the developers of Figlet for their incredible open-source contribution.

