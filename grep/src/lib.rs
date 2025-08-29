use std::{
    fs::File,
    io::{BufRead, BufReader, Result},
};

#[derive(Debug)]
pub struct Match {
    pub filename: String,
    pub line_number: usize,
    pub line: String,
}

pub struct PatternMatcher<'a> {
    patterns: &'a [String],
    filenames: &'a [String],
    case_sensitive: bool,
    invert_match: bool,
}

impl<'a> PatternMatcher<'a> {
    pub fn new(patterns: &'a [String], filenames: &'a [String]) -> Self {
        Self {
            patterns,
            filenames,
            case_sensitive: true,
            invert_match: false,
        }
    }

    pub fn case_sensitive(mut self, case_sensitive: bool) -> Self {
        self.case_sensitive = case_sensitive;
        self
    }

    pub fn invert_match(mut self, invert_match: bool) -> Self {
        self.invert_match = invert_match;
        self
    }

    pub fn find_matches(&self) -> Result<Vec<Match>> {
        let mut results = Vec::new();
        for filename in self.filenames {
            let file = File::open(filename)?;
            let reader = BufReader::new(file);
            for (line_number, line_result) in reader.lines().enumerate() {
                let line = line_result?;
                let matches = if self.case_sensitive {
                    self.patterns.iter().any(|p| line.contains(p))
                } else {
                    self.patterns
                        .iter()
                        .any(|p| line.to_lowercase().contains(&p.to_lowercase()))
                };
                if matches ^ self.invert_match {
                    results.push(Match {
                        filename: filename.to_string(),
                        line_number: line_number + 1,
                        line,
                    });
                }
            }
        }
        Ok(results)
    }
}
