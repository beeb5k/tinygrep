use std::process::exit;

struct SearchQuery<'a> {
    term: &'a str,
    filenames: Vec<&'a str>,
}

impl<'a> SearchQuery<'a> {
    fn new(term: &'a str, filenames: Vec<&'a str>) -> Self {
        Self { term, filenames }
    }

    fn search<'b>(&self, file_content: &'b str) -> Vec<&'b str> {
        if file_content.is_empty() {
            eprintln!("File is empty");
            exit(1)
        }

        file_content
            .lines()
            .filter(|line| line.contains(self.term))
            .collect()
    }
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    // 1 args is binry name
    if args.len() < 3 {
        eprintln!("Not enought args");
        exit(1)
    }

    let term = &args[1];
    let filenames: Vec<&str> = args[2..].iter().map(|s| s.as_str()).collect();

    let query = SearchQuery::new(term, filenames);

    // ? for now only reading first file. will impliment multiple files later.
    let file_content =
        std::fs::read_to_string(query.filenames[0]).expect("Failed to read the file");

    let result = query.search(&file_content);
    match result.is_empty() {
        true => {
            eprintln!("No matches found");
            exit(1)
        }
        false => {
            for line in result {
                println!("{line}");
            }
        }
    }
}
