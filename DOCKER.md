# Using Quoin with Docker

The easiest way to use Quoin is via its official Docker image. This ensures you have all prerequisites (Pandoc, Typst, LaTeX fonts) ready to go.

## 1. Pull the Image

```bash
docker pull ghcr.io/bilal-houari/quoin:latest
```

## 2. Using the Web Interface (Live Preview)

Run the server and access it at `http://localhost:3232`:

```bash
docker run -it --init --rm -p 3232:3232 ghcr.io/bilal-houari/quoin:latest server --allow-external
```

## 3. Using the CLI

To convert local files, mount your current directory to `/data` inside the container:

### Convert a file
```bash
docker run -it --init --rm -v $(pwd):/data ghcr.io/bilal-houari/quoin:latest convert input.md -o output.pdf
```

### Advanced options (Dense layout + LaTeX fonts)
```bash
docker run -it --init --rm -v $(pwd):/data ghcr.io/bilal-houari/quoin:latest convert input.md --dense --latex-font
```

## Troubleshooting

- **Permissions**: Ensure your Markdown files are readable by the container.
- **Paths**: When using the CLI, always refer to files relative to the directory you mounted.
