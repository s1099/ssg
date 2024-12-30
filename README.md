# Static Site Generator

Static site generator built in rust that generates HTML from markdown

## Usage

1. Add Markdown files in `content/`
2. Add a `base.html` template in the `templates/` directory
3. Run generator with `cargo run`

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
- [Rust](https://rustup.rs/)
