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
    match tera.render("index.html", &context) {
        Ok(rendered) => println!("{}", rendered),
        Err(e) => eprintln!("Rendering error: {}", e),
    }
}

