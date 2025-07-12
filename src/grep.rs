use std::fs::File;
use std::io::{self, BufRead, BufReader, Write};
use regex::Regex;
use termcolor::{Color, ColorChoice, ColorSpec, StandardStream, WriteColor};

pub struct Grep;

impl Grep {
    /**
     * Function that highlights and bolds any matches of the regex in the given line,
     * then writes the line to the standard output.
     */
    fn highlight(&self, line: &str, regex: &Regex, stdout: &mut StandardStream) {
        let mut last_end = 0;

        for mat in regex.find_iter(line) {
            let (start, end) = (mat.start(), mat.end());

            write!(stdout, "{}", &line[last_end..start]).unwrap();

            stdout
                .set_color(ColorSpec::new().set_fg(Some(Color::Red)).set_bold(true))
                .unwrap();

            write!(stdout, "{}", &line[start..end]).unwrap();
            stdout.reset().unwrap();

            last_end = end;
        }

        writeln!(stdout, "{}", &line[last_end..]).unwrap();
    }

    /**
     *  Function that processes a file by reading it line by line,
     *  and calls the highlight function for each line that matches the regex.
     */
    fn process(&self, regex: Regex, path: String) {
        let file = match File::open(&path) {
            Ok(f) => f,
            Err(e) => {
                eprintln!("❌ Error: Could not open file '{}': {}", path, e);
                return;
            }
        };

        let reader = BufReader::new(file);
        let mut stdout = StandardStream::stdout(ColorChoice::Always);

        let mut any_matches = false;

        for line in reader.lines() {
            let line = line.unwrap();
            if regex.is_match(&line) {
                any_matches = true;
                self.highlight(&line, &regex, &mut stdout);
            }
        }

        if !any_matches {
            writeln!(stdout, "No matches found.").unwrap();
        }
    }

    /**
     * Main grep function that reads the user input from the command line,
     * parses the regex patter and file path,
     * then calls the process function to handle the file.
     */
    pub fn run(&self) {
        let stdin = io::stdin();
        let mut stdout = io::stdout();
        let mut input = String::new();

        loop {
            print!("CMD> ");
            stdout.flush().unwrap();
            input.clear();
            stdin.read_line(&mut input).unwrap();

            let args: Vec<&str> = input.trim().split_whitespace().collect();

            if args.is_empty() {
                continue;
            }

            if args[0] == "exit" {
                break;
            }

            if args.len() < 3 {
                eprintln!("Usage: mygrep <pattern> <file>");
                continue;
            }

            let pattern = args[1];
            let file_path = args[2];

            let regex = match Regex::new(pattern) {
                Ok(r) => r,
                Err(e) => {
                    eprintln!("Invalid regex pattern: {}", e);
                    continue;
                }
            };

            self.process(regex, file_path.to_string());
        }
    }
}
