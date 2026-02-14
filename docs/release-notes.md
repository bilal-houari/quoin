# Release Documentation

This release provides pre-compiled binaries for Linux, macOS (Intel and Apple Silicon), and Windows. Please review the system requirements and installation instructions below to ensure proper functionality.

## System Requirements

The application depends on two external components that must be installed and available in the system PATH:

1. **Typst**: The latest stable version is recommended.
2. **Pandoc**: Version 3.0 or higher is required, the latest stable version is recommended.

## Installation Instructions

### macOS and Linux
The recommended method for installation on macOS and Linux is via Homebrew. Execute the following command in your terminal:

```bash
brew install typst pandoc
```

### Windows
For Windows environments, the following installation methods are recommended:

*   **Pandoc**: Download and execute the official MSI installer from the [Pandoc installation page](https://pandoc.org/installing.html). Ensure the version is 3.0 or higher.
*   **Typst**: Install via the Windows Package Manager (winget) by executing the following command in PowerShell:

```powershell
winget install --id Typst.Typst
```

## Post-Installation
After installing the required dependencies, it may be necessary to restart your terminal session or log out and back in to ensure the system PATH is updated. You can verify the installations by running the following commands:

```bash
typst --version
pandoc --version
```