use clap::Parser;
use std::fs;
use std::path::Path;
use walkdir::WalkDir;

#[derive(Parser)]
#[command(name = "rc")]
#[command(about = "Remove comments from JavaScript and TypeScript files")]
struct Cli {
    /// Path to file or directory to process
    path: String,
    
    /// Process files in-place (modify original files)
    #[arg(short, long)]
    in_place: bool,
    
    /// Verbose output
    #[arg(short, long)]
    verbose: bool,
}

fn main() {
    let cli = Cli::parse();
    
    let path = Path::new(&cli.path);
    
    if path.is_file() {
        if is_js_or_ts_file(path) {
            process_file(path, cli.in_place, cli.verbose);
        } else {
            eprintln!("Error: File must be a JavaScript (.js) or TypeScript (.ts) file");
            std::process::exit(1);
        }
    } else if path.is_dir() {
        process_directory(path, cli.in_place, cli.verbose);
    } else {
        eprintln!("Error: Path '{}' does not exist", cli.path);
        std::process::exit(1);
    }
}

fn is_js_or_ts_file(path: &Path) -> bool {
    if let Some(extension) = path.extension() {
        let ext = extension.to_string_lossy().to_lowercase();
        ext == "js" || ext == "ts" || ext == "jsx" || ext == "tsx"
    } else {
        false
    }
}

fn process_directory(dir: &Path, in_place: bool, verbose: bool) {
    let mut processed_count = 0;
    
    for entry in WalkDir::new(dir) {
        match entry {
            Ok(entry) => {
                let path = entry.path();
                if path.is_file() && is_js_or_ts_file(path) {
                    process_file(path, in_place, verbose);
                    processed_count += 1;
                }
            }
            Err(e) => {
                eprintln!("Error reading directory entry: {}", e);
            }
        }
    }
    
    if verbose {
        println!("Processed {} files", processed_count);
    }
}

fn process_file(path: &Path, in_place: bool, verbose: bool) {
    if verbose {
        println!("Processing: {}", path.display());
    }
    
    let content = match fs::read_to_string(path) {
        Ok(content) => content,
        Err(e) => {
            eprintln!("Error reading file '{}': {}", path.display(), e);
            return;
        }
    };
    
    let cleaned_content = remove_comments(&content);
    
    if in_place {
        match fs::write(path, cleaned_content) {
            Ok(_) => {
                if verbose {
                    println!("Updated: {}", path.display());
                }
            }
            Err(e) => {
                eprintln!("Error writing file '{}': {}", path.display(), e);
            }
        }
    } else {
        print!("{}", cleaned_content);
    }
}

fn remove_comments(content: &str) -> String {
    let mut result = String::new();
    let mut in_string = false;
    let mut in_template_literal = false;
    let mut string_char = '\0';
    let mut escape_next = false;
    let mut chars = content.chars().peekable();
    
    while let Some(ch) = chars.next() {
        if escape_next {
            result.push(ch);
            escape_next = false;
            continue;
        }
        
        if ch == '\\' && (in_string || in_template_literal) {
            result.push(ch);
            escape_next = true;
            continue;
        }
        
        if !in_string && !in_template_literal {
            // Check for template literal start
            if ch == '`' {
                in_template_literal = true;
                result.push(ch);
                continue;
            }
            
            // Check for string start
            if ch == '"' || ch == '\'' {
                in_string = true;
                string_char = ch;
                result.push(ch);
                continue;
            }
            
            // Check for single-line comment
            if ch == '/' && chars.peek() == Some(&'/') {
                // Skip until end of line
                while let Some(next_ch) = chars.next() {
                    if next_ch == '\n' {
                        result.push(next_ch);
                        break;
                    }
                }
                continue;
            }
            
            // Check for multi-line comment
            if ch == '/' && chars.peek() == Some(&'*') {
                chars.next(); // consume the '*'
                let mut found_end = false;
                
                while let Some(next_ch) = chars.next() {
                    if next_ch == '*' && chars.peek() == Some(&'/') {
                        chars.next(); // consume the '/'
                        found_end = true;
                        break;
                    }
                }
                
                if !found_end {
                    // Unclosed comment, this shouldn't happen in valid JS/TS
                    eprintln!("Warning: Unclosed multi-line comment found");
                }
                continue;
            }
        }
        
        // Handle string/template literal endings
        if in_string && ch == string_char {
            in_string = false;
            string_char = '\0';
        } else if in_template_literal && ch == '`' {
            in_template_literal = false;
        }
        
        result.push(ch);
    }
    
    // Clean up multiple consecutive empty lines
    let lines: Vec<&str> = result.lines().collect();
    let mut cleaned_lines = Vec::new();
    let mut empty_line_count = 0;
    
    for line in lines {
        if line.trim().is_empty() {
            empty_line_count += 1;
            if empty_line_count <= 1 {
                cleaned_lines.push(line);
            }
        } else {
            empty_line_count = 0;
            cleaned_lines.push(line);
        }
    }
    
    cleaned_lines.join("\n")
}
