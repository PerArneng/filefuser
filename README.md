![example workflow](https://github.com/perarneng/filefuser/actions/workflows/release.yaml/badge.svg)

![filefuser logo](docs/filefuser-logo.png)

# filefuser

This Rust tool aggregates text files into a single EML archive. It scans a specified directory for 
files matching user-defined glob patterns, extracts metadata to determine if they are valid 
text files, and then combines them into a MIME multipart message complete with generated 
headers and boundaries.

## Installation

On MacOS or Linux you can install filefuser using Homebrew:
```bash
brew install perarneng/tap/filefuser
```
If you are on Windows or use other package manager then download appropriate binary from the release page.

## Architecture

This project is designed as a modular, concurrent command‐line tool that aggregates text files into a 
single MIME multipart EML archive. Its architecture is divided into distinct layers: the CLI and configuration 
layer parses command-line arguments using the Clap library, ensuring that file paths, 
search directories, and glob patterns are validated and correctly set up. The directory 
scanning module recursively walks the file system, employing regex-based glob matching to 
efficiently identify candidate files. Once located, the file data extraction layer leverages 
asynchronous operations with Tokio to concurrently analyze files, extract metadata such as 
file size and content type (text or binary), and gracefully capture errors without halting 
overall processing. Finally, the archiving layer encapsulates the logic for constructing the 
EML archive—it dynamically generates MIME headers and boundaries, reads each text file's content, 
and assembles everything into a well-formed EML file. Throughout the system, dedicated logging and 
utility modules support detailed error reporting and streamlined file operations, resulting in a 
robust, maintainable, and high-performance Rust application.
