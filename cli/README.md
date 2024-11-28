# pulldown-html-ext-cli

A command-line tool for converting markdown to HTML using the pulldown-html-ext library.

## Installation

```bash
cargo install pulldown-html-ext-cli
```

## Usage

Basic usage:
```bash
# Read from stdin, write to stdout
echo "# Hello" | pulldown-html-ext-cli

# Convert a file
pulldown-html-ext-cli -i input.md -o output.html

# Use custom configuration
pulldown-html-ext-cli -i input.md -o output.html -c config.toml
```

## Options

```
Options:
  -i, --input <FILE>    Input markdown file (omit for stdin)
  -o, --output <FILE>   Output HTML file (omit for stdout)
  -c, --config <FILE>   Config file in TOML format
  -h, --help           Print help
  -V, --version        Print version
```

## Configuration

You can customize the HTML output by providing a TOML configuration file. Example configuration:

```toml
[html]
escape_html = true
break_on_newline = true
xhtml_style = false
pretty_print = true

[elements.headings]
add_ids = true
id_prefix = "heading-"

[elements.links]
nofollow_external = true
open_external_blank = true

[elements.code_blocks]
default_language = "text"
line_numbers = true
```

For full configuration options, see the [library documentation](https://docs.rs/pulldown-html-ext).

## Example

```bash
# Using stdin/stdout
echo "# Hello World" | pulldown-html-ext-cli > output.html

# Using files with custom config
pulldown-html-ext-cli -i document.md -o document.html -c custom.toml
```

## License

[Add your license information here]
