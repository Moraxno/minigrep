use std::error::Error;
use std::fs;

#[derive(Debug)]
pub struct Config {
    pub query: String,
    pub file_path: String,
    pub ignore_case: bool,
}

#[derive(Debug, PartialEq)]
pub struct TextMatch<'a> {
    pub line: &'a str,
    pub line_num: usize,
    pub start: usize,
    pub end: usize,
}

impl Config {
    pub fn build(args: &[String], ignore_case: bool) -> Result<Config, &'static str> {
        // --snip--
        if args.len() < 3 {
            return Err("not enough arguments");
        }

        let query = args[1].clone();
        let file_path = args[2].clone();

        Ok(Config {
            query,
            file_path,
            ignore_case,
        })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    // --snip--
    let contents = fs::read_to_string(config.file_path)?;

    let results = search(config.query.as_str(), contents.as_str(), config.ignore_case);

    for result in results {
        let (prefix, rest) = result.line.split_at(result.start);
        let (word, suffix) = rest.split_at(result.end - result.start);

        println!(
            "L{}C{}: {}\x1b[1m{}\x1b[0m{}",
            result.line_num, result.start, prefix, word, suffix
        );
    }

    Ok(())
}

pub fn search<'a>(query: &str, contents: &'a str, ignore_case: bool) -> Vec<TextMatch<'a>> {
    let mut result = vec![];
    let mut search_text: String;

    let search_query = match ignore_case {
        true => String::from(query),
        false => query.to_lowercase(),
    };

    for (line_num, line) in contents.lines().enumerate() {
        let mut read_head: usize = 0;
        if ignore_case {
            search_text = line.to_lowercase();
        } else {
            search_text = String::from(line);
        }
        while let Some(start) = search_text.find(search_query.as_str()) {
            result.push(TextMatch {
                line: line,
                line_num: line_num,
                start: read_head + start,
                end: read_head + start + search_query.len(),
            });
            read_head += start + query.len();
            let (_, new_search_str) = line.split_at(read_head);
            search_text = new_search_str.to_string();
        }
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_result() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.";

        assert_eq!(
            TextMatch {
                line: "safe, fast, productive.",
                line_num: 1,
                start: 15,
                end: 19
            },
            search(query, contents)[0]
        );
    }
}
