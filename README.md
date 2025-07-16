# RC - Remove Comments

A simple CLI tool that removes comments from JavaScript and TypeScript files.

## Features

- Remove single-line comments (`//`)
- Remove multi-line comments (`/* */`)
- Process single files or directories recursively
- Supports JavaScript (`.js`, `.jsx`) and TypeScript (`.ts`, `.tsx`) files
- In-place editing option
- Preserves strings and template literals containing comment-like syntax
- Handles escape sequences properly

## Installation

```bash
cargo build --release
```

## Usage

### Process a single file (output to stdout)
```bash
rc path/to/file.js
```

### Process a file in-place (modify the original file)
```bash
rc --in-place path/to/file.js
```

### Process a directory recursively
```bash
rc path/to/directory
```

### Process with verbose output
```bash
rc --verbose path/to/directory
```

### Combine options
```bash
rc --in-place --verbose path/to/directory
```

## Options

- `-i, --in-place`: Process files in-place (modify original files)
- `-v, --verbose`: Verbose output showing which files are being processed
- `-h, --help`: Show help message

## Examples

```bash
# Remove comments from a single file and print to stdout
rc src/main.js

# Remove comments from all JS/TS files in a directory
rc --in-place --verbose src/

# Process specific file types in a project
rc --in-place --verbose .
```

## What it removes

- Single-line comments: `// comment`
- Multi-line comments: `/* comment */`
- Inline comments: `code(); // comment`
- Block comments with multiple lines

## What it preserves

- Comment-like syntax inside strings: `"This has // inside"`
- Comment-like syntax inside template literals: `` `This has /* inside */ too` ``
- Code structure and formatting (except empty lines are cleaned up)

## Supported file extensions

- `.js` - JavaScript files
- `.jsx` - React JavaScript files
- `.ts` - TypeScript files
- `.tsx` - React TypeScript files
