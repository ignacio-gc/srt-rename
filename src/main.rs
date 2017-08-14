extern crate regex;
extern crate glob;
use regex::Regex;
use std::fs;
use glob::glob;

fn main() {

    fn iter_path(path: Result<Paths, PatternError>) -> () {
        
    }

    fn foo() -> std::io::Result<()> {

        let re = Regex::new(r".*([sS][0-9][0-9][eE][0-9][0-9]).*").unwrap();

        for entry in glob("*.srt").expect("Failed to read glob pattern") {
            match entry {
                Ok(path) => if re.is_match(&path) {
                    println!("{:?}", path);
                },
                Err(e) => println!("{:?}", e)
            }
        }
        Ok(())
    }

    foo();
}

