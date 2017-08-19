extern crate regex;
extern crate glob;
//extern crate globset;
use regex::Regex;
use glob::glob;
use glob::glob_with;
use glob::MatchOptions;
//use globset::{Glob, GlobSetBuilder};

fn main() {

    let re = Regex::new(r".*([sS][0-9][0-9][eE][0-9][0-9]).*").unwrap();

    for video in glob("*.avi").expect("Failed to read glob pattern") {
        match video {
            Ok(path) => {
                println!("path {}", path.to_str().unwrap());
                if re.is_match(&path.to_str().unwrap()) {

                    let season_and_episode = re.captures(path.to_str().unwrap())
                        .unwrap()
                        .get(1)
                        .map_or("", |m| m.as_str());

                    let options = MatchOptions {
                        case_sensitive: false,
                        require_literal_separator: false,
                        require_literal_leading_dot: false,
                    };

                    let str_glob = format!("*{}*.srt", season_and_episode);
                    println!("{}", str_glob);

                    for srt in glob_with(&str_glob, &options).expect("No srt file") {
                        std::fs::rename(srt.unwrap(), path.with_extension("srt")).unwrap();
                    }
                }
            },
            Err(e) => println!("{:?}", e)
        }
    }
}
