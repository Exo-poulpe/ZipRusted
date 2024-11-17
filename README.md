# ZipRusted

**ZipRusted** is a simple monothreaded ZIP archive bruteforcer written in Rust. It attempts to decrypt password-protected ZIP files using a wordlist of possible passwords.

## Features

- **Brute-force ZIP Archives**: Use a wordlist to attempt to decrypt password-protected ZIP files.
- **Verbose Mode**: Option to display detailed output during the bruteforce process.
- **Simple Command-Line Interface**: Easy to use with straightforward command-line options.


## Usage

   ```bash
Simple monothread ZIP bruteforcer in Rust

Usage: ziprusted [OPTIONS] --file <FILE> --wordlist <WORDLIST>

Options:
-f, --file <FILE>          Filepath of the archive to bruteforce
-w, --wordlist <WORDLIST>  Wordlist filepath
-v, --verbose              Enable verbose mode
-h, --help                 Print help
   ```

### Quick start 

   ```bash
   .\ziprusted -f archive_zip.zip -w wordlist.txt
   ```



## Installation

### Prerequisites

- **Rust**: Ensure you have Rust and Cargo installed. If not, download them from the [official website](https://www.rust-lang.org/tools/install).

### Build from Source

1. **Clone the Repository**

```bash
git clone https://github.com/yourusername/ZipRusted.git
cd ZipRusted
```

2. **Compile with cargo**

```bash
cargo build -r
```

3. **Profit**

```bash
.\ziprusted -f archive_zip.zip -w wordlist.txt -v
```