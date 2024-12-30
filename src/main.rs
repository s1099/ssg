use pulldown_cmark::{Parser, html, Event, Tag, HeadingLevel};
use std::fs::{self, File};
use std::io::{self, Write};
use glob::glob;
use tera::Tera;

fn main() -> io::Result<()> {
    let tera = match Tera::new("templates/**/*.html") {
        Ok(t) => t,
        Err(e) => {
            println!("Error in parsing: {}", e);
            std::process::exit(1);
        }
    };

    fs::create_dir_all("public").map_err(|e| {
        eprintln!("Unable to make directory: {}", e);
        e
    })?;

    for entry in glob("content/*.md").expect("Failed to read glob pattern") {
        match entry {
            Ok(path) => {
                let file_name = path.file_stem().unwrap().to_str().unwrap();
                let md = fs::read_to_string(&path)?;
                let title = get_title(&md);
                let parser = Parser::new(&md);

                let mut html_out = String::new();
                html::push_html(&mut html_out, parser);

                let mut ctx = tera::Context::new();
                ctx.insert("title", &title);
                ctx.insert("content", &html_out);
                let rendered = match tera.render("base.html", &ctx) {
                    Ok(rendered) => rendered,
                    Err(e) => {
                        eprintln!("Failed to render template: {}", e);
                        std::process::exit(1);
                    }
                };
                
                let out_path = format!("public/{}.html", file_name);
                let mut out_file = File::create(out_path)?;
                write!(out_file, "{}", rendered)?;
            }
            Err(e) => eprintln!("{:?}", e),
        }
    }

    Ok(())
}

fn get_title(markdown: &str) -> String {
    let mut parser = Parser::new(markdown);
    while let Some(event) = parser.next() {
        if let Event::Start(Tag::Heading { level: HeadingLevel::H1, .. }) = event {
            if let Some(Event::Text(text)) = parser.next() {
                return text.to_string();
            }
        }
    }
    "Untitled".to_string()
}
