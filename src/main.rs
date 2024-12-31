use clap::{Arg, Command};
use glob::glob;
use pulldown_cmark::{html, Event, HeadingLevel, Parser, Tag};
use std::fs::{self, File};
use std::io::{self, Write};
use tera::Tera;

fn main() -> io::Result<()> {
    let mut cmd = Command::new("SSG")
        .about("Static site generator")
        .arg(
            Arg::new("input_dir")
                .short('i')
                .long("input")
                .value_name("DIR")
                .help("Sets the directory containing md files")
                .default_value("content"),
        )
        .arg(
            Arg::new("template_dir")
                .short('t')
                .long("templates")
                .value_name("DIR")
                .help("Sets the directory containing templates")
                .default_value("templates"),
        )
        .arg(
            Arg::new("out_dir")
                .short('o')
                .long("output")
                .value_name("DIR")
                .help("Sets the directory to output HTML files")
                .default_value("public"),
        )
        .arg(
            Arg::new("html_template")
                .short('t')
                .long("templaye")
                .value_name("FILE")
                .help("Sets the HTML template to use")
                .default_value("base.html"),
        );
    let matches = cmd.get_matches_mut();

    let input_dir = matches.get_one::<String>("input_dir").unwrap();
    let template_dir = matches.get_one::<String>("template_dir").unwrap();
    let out_dir = matches.get_one::<String>("out_dir").unwrap();
    let html_template = matches.get_one::<String>("html_template").unwrap();
    
    let tera = match Tera::new(&format!("{}/*.html", template_dir)) {
        Ok(t) => t,
        Err(e) => {
            println!("Error in parsing: {}", e);
            std::process::exit(1);
        }
    };

    fs::create_dir_all(out_dir).map_err(|e| {
        eprintln!("Unable to make directory: {}", e);
        e
    })?;

    let entries: Vec<_> = glob(&format!("{}/*.md", input_dir))
        .expect("Failed to read glob pattern")
        .collect();

    if entries.is_empty() {
        println!("No markdown files found in '{}' directory.", input_dir);
        cmd.print_help().expect("Failed to print help");
        return Ok(());
    }

    for entry in entries {
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
                let rendered = match tera.render(html_template, &ctx) {
                    Ok(rendered) => rendered,
                    Err(e) => {
                        eprintln!("Failed to render template: {}", e);
                        std::process::exit(1);
                    }
                };

                let out_path = format!("{}/{}.html", out_dir, file_name);
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
        if let Event::Start(Tag::Heading {
            level: HeadingLevel::H1,
            ..
        }) = event
        {
            if let Some(Event::Text(text)) = parser.next() {
                return text.to_string();
            }
        }
    }
    "Untitled".to_string()
}
