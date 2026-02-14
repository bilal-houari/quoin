# Quoin

Quoin is a document engine that turns Markdown files into professional PDFs. It acts as a **wrapper around Pandoc and Typst**, using Pandoc to parse your documents and Typst to handle the high-quality layout and typesetting.

## Features

*   **Pandoc & Typst Wrapper**: Combines the document conversion of Pandoc with the modern layout engine of Typst.
*   **Layout Presets**: Quick options for document density (ultra-dense, dense, standard, or comfort).
*   **Styling**: Support for LaTeX-style fonts (New Computer Modern), section numbering, two-column layouts, etc.
*   **Live Preview**: A built-in web server to edit Markdown and see PDF changes in real-time.
*   **Flexible**: Export directly to PDF or get the intermediate Typst source code.

## Screenshots

<img width="1920" height="1033" alt="image of the web interface of the app" src="https://github.com/user-attachments/assets/0ee78491-4e97-43fc-99a3-05c49bfc5f86" />

## Quick Start (Docker)

If you have Docker installed, you can start the web server immediately:

```bash
docker run -it --init --rm -p 3232:3232 ghcr.io/bilal-houari/quoin:latest server --allow-external
```

For detailed Docker usage (CLI, mounting files, etc.), see [DOCKER.md](DOCKER.md).

## Prerequisites

You will need the following installed:

*   [Pandoc](https://pandoc.org/)
*   [Typst](https://typst.app/)
*   [Rust](https://www.rust-lang.org/) (to build the CLI)
*   [Node.js](https://nodejs.org/) (to build the web interface)

## Building from Source

1.  **Build the Web Interface**:
    ```bash
    cd web
    npm install
    npm run build
    cd ..
    ```

2.  **Build the Tool**:
    ```bash
    cargo build --release
    ```
    The binary will be at `target/release/quoin`.

## Usage

### CLI
Convert Markdown to PDF:
```bash
quoin convert input.md -o output.pdf
```

Apply layout options like dense text and LaTeX fonts:
```bash
quoin convert input.md --dense --latex-font --section-numbering
```

Output the Typst file instead of a PDF:
```bash
quoin convert input.md --typ
```

### Web Interface
To start the live preview server (defaulting to port 3000):
```bash
quoin server
```

## Options

*   `--ultra-dense`: Uses 8pt font and 2cm margins.
*   `--two-cols`: Sets the document to a two-column layout.
*   `--outline`: Adds a Table of Contents at the end.
*   `-V key=value`: Sets custom variables for the Typst template.
*   And more.
