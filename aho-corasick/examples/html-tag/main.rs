use std::{env, fs, io::Write};
use std::fs::File;

use aho_corasick::aho_corasick::AhoCorasick;

const OPEN_BODY: &str = "<body style=\"width: 70%;white-space: initial;word-wrap: break-word;\">";
const CLOSE_BODY: &str = "</body>";
const OPEN_TAG: &str = "<span style=\"background-color:powderblue;\">";
const CLOSE_TAG: &str = "</span>";

fn main() {
    let mut output: String = String::new();
    let contents = fs::read_to_string("./examples/html-tag/input.txt")
        .expect("Should have been able to read the file");

    let args = Vec::from_iter(env::args());
    let patterns: Vec<&str> = args[1..].iter().map(|v| v.as_ref()).collect::<Vec<_>>();
    
    let mut ac = AhoCorasick::new(&patterns);
    ac.search_text(&contents);

    let mut last_pattern: usize = 0;

    output.push_str(OPEN_BODY);
    for match_result in ac.result.iter() {
        output.push_str(&contents[last_pattern..match_result.start as usize]);
        output.push_str(OPEN_TAG);
        output.push_str(&contents[match_result.start as usize..match_result.end as usize + 1]);
        output.push_str(CLOSE_TAG);
        last_pattern = match_result.end as usize + 1;
    }
    output.push_str(CLOSE_BODY);
    
    let _ = match File::create("./examples/html-tag/output.html") {
        Ok(mut file) =>  file.write_all(output.as_bytes()),
        Err(err) => Err(err),
    };
}
