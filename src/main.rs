use std::{
    fs::File,
    io::{BufRead, BufReader},
    process::exit,
};

struct SearchQuery<'a> {
    term: &'a str,
    filenames: Vec<&'a str>,
}

impl<'a> SearchQuery<'a> {
    fn new(term: &'a str, filenames: Vec<&'a str>) -> Self {
        Self { term, filenames }
    }

    fn search(&self, reader: BufReader<File>) -> Vec<String> {
        reader
            .lines()
            .filter_map(|line_result| match line_result {
                Ok(line) if line.contains(self.term) => Some(line),
                _ => None,
            })
            .collect()
    }
}

fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() < 3 {
        eprintln!("Usage: {} <search_term> <file1> [file2...]", args[0]);
        exit(1);
    }

    let term = &args[1];
    let filenames: Vec<&str> = args[2..].iter().map(|s| s.as_str()).collect();
    let query = SearchQuery::new(term, filenames);

    let mut i = 0;
    while i < query.filenames.len() {
        let file = File::open(query.filenames[i])
            .unwrap_or_else(|_| panic!("Failed to open file {}", query.filenames[i]));

        let reader = BufReader::new(file);
        let result = query.search(reader);

        if result.is_empty() {
            eprintln!("No matches found for {}", query.filenames[i]);
        }

        for line in result {
            match query.filenames.len() {
                1 => println!("{line}"),
                _ => println!("{:<15}{}", query.filenames[i], line),
            }
        }
        i += 1;
    }
}
