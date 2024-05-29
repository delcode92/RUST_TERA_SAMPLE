use std::fs::File;
use std::io::Write;
use tera::{Context, Tera};

fn main() {
    // Load templates from the `templates` directory
    let tera = match Tera::new("templates/**/*") {
        Ok(t) => t,
        Err(e) => {
            eprintln!("Parsing error(s): {}", e);
            ::std::process::exit(1);
        }
    };

    // Create a context and insert values
    let mut context = Context::new();
    context.insert("name", "Rust");

    // Render the `index.html` template with the context
    let rendered = match tera.render("index.html", &context) {
        Ok(output) => output,
        Err(e) => {
            eprintln!("Rendering error: {}", e);
            ::std::process::exit(1);
        }
    };

    // Write the rendered output to a new file
    let output_path = "output2.html";
    match write_to_file(output_path, &rendered) {
        Ok(_) => println!("Successfully wrote to {}", output_path),
        Err(e) => eprintln!("Error writing to file: {}", e),
    }
}

fn write_to_file(path: &str, content: &str) -> std::io::Result<()> {
    let mut file = File::create(path)?;
    file.write_all(content.as_bytes())
}

