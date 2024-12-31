# Static Site Generator

Static site generator built in rust that generates HTML from markdown

## Usage

1. Build using `cargo build --release`
2. Copy the ssg binary in `target/release/` to a dir in PATH
3. Run `ssg`, use `--help` flag to print help

### Example `base.html`
```html
<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>{{ title }}</title>
</head>
<body>
    <div>{{ content | safe }}</div>
</body>
</html>
```

## Requirements
- [Rust and Cargo](https://rustup.rs/)
