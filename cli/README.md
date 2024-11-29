Here is the rewritten README for the pulldown-html-ext-cli crate:

# pulldown-html-ext-cli

A command-line tool for converting Markdown to HTML using the pulldown-html-ext library.

## Installation

You can install the pulldown-html-ext-cli tool using Cargo:

```bash
cargo install pulldown-html-ext-cli
```

## Usage

The tool supports several options for converting Markdown to HTML:

```
pulldown-html-ext-cli [OPTIONS]
```

### Options

- `-i, --input <FILE>`: Specify the input Markdown file. If omitted, the tool will read from standard input.
- `-o, --output <FILE>`: Specify the output HTML file. If omitted, the tool will write to standard output.
- `-c, --config <FILE>`: Provide a TOML configuration file to customize the HTML output.
- `-h, --help`: Display the help message.
- `-V, --version`: Print the version information.

### Basic Usage

To convert a Markdown file to HTML and write the output to stdout:

```bash
pulldown-html-ext-cli -i input.md
```

To convert a Markdown file and write the HTML output to a file:

```bash
pulldown-html-ext-cli -i input.md -o output.html
```

### Using a Configuration File

You can provide a TOML configuration file to customize the HTML output. Here's an example configuration:

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

To use the custom configuration, run the tool with the `-c` option:

```bash
pulldown-html-ext-cli -i input.md -o output.html -c custom.toml
```

For more information on the available configuration options, please refer to the [pulldown-html-ext library documentation](https://docs.rs/pulldown-html-ext).

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.
