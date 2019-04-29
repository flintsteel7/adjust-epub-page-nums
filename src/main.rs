#![allow(dead_code)]
#![allow(unused_variables)]

#[macro_use] extern crate lazy_static;
extern crate regex;

use regex::Regex;
use read_write_files::*;

fn main() {
//    let path = "/Users/pxturne/Documents/EPUB Projects/Test";
    let path = std::env::args().nth(1).expect("no path given");
    let error = format!("Could not read {:?}", path);
    let mut file_data = read_files(&path, "xhtml").expect(&error);
    for file in file_data.iter_mut() {
        match change_file_contents(file) {
            Some(new_file) => *file = new_file,
            None => continue
        }
    }
    for result in write_files(file_data) {
        match result {
            Ok(filename) => println!("Changes written to {:?}", filename),
            Err((filename, error)) => println!("Encountered a problem writing {:?}: {:?}", filename, error)
        };
    }
}

fn change_file_contents(file: &FileData) -> Option<FileData> {
    let mut changed = false;
    let mut new_contents = file.contents.clone();

    if let Some(surrounded_plain_text) = parse_page_nums(&new_contents) {
        new_contents = surrounded_plain_text;
        changed = true;
    }

    if changed {
        Some(FileData { contents: new_contents, ..file.clone() })
    } else {
        None
    }
}

fn parse_page_nums(file_contents: &str) -> Option<String> {
    lazy_static! {
        static ref PAGEBREAK: Regex = Regex::new(r#"(?P<entire><span epub:type="pagebreak" id="page\d+" title="(?P<orignum>\d+)"></span><!-- (?P<operation>\w+) (?P<amount>\d+) -->)"#).unwrap();
    }

    if PAGEBREAK.is_match(file_contents) {
        let mut new_contents = String::from(file_contents);

        for caps in PAGEBREAK.captures_iter(file_contents) {
            let cap = &caps["entire"];
            let orignum = &caps["orignum"].parse::<i32>().unwrap();
            let amount = &caps["amount"].parse::<i32>().unwrap();
            let newint = perform_operation(*orignum, &caps["operation"], *amount);
            let newnum = int_to_pagenum(newint);

            let replacement = format!("<span epub:type=\"pagebreak\" id=\"page{}\" title=\"{}\"></span>", newnum, newnum);

            new_contents = new_contents.replacen(&cap, &replacement, 1);
        }
        Some(String::from(new_contents))
    } else {
        None
    }
}

fn perform_operation(orignum: i32, operation: &str, amount: i32) -> i32 {
    match operation {
        "subtract" => orignum - amount,
        "add" => orignum + amount,
        _ => orignum,
    }
}

fn int_to_pagenum(int: i32) -> String {
    if int > 0 {
        int.to_string()
    } else {
        "i".to_string()
    }
}