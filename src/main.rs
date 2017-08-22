extern crate regex;
use std::io;
use std::fs;
use std::path::PathBuf;
use regex::Regex;
use regex::RegexBuilder;

fn main() {

    fn find_srt_and_rename(path: PathBuf,
                           re_srt: &Regex,
                           season_and_episode: &Regex) -> io::Result<()> {
        for entry in fs::read_dir("./").unwrap() {
            match entry {
                Ok(e) => {

                    let srt_path = e.path();

                    if re_srt.is_match(&srt_path.to_str().unwrap())
                        && season_and_episode.is_match(&srt_path.to_str().unwrap()
                        ) {
                            if e.path() != path.as_path().with_extension("srt").as_path() {
                                std::fs::rename(e.path(), path.as_path().with_extension("srt").as_path())?;
                                println!("{} --> {}",
                                         e.path().to_str().unwrap(),
                                         path.as_path().with_extension("srt").to_str().unwrap());
                            } else {
                                println!(" = El archivo ya tiene el nombre correcto");
                            }
                            return Ok(())
                        }
                },
                Err(error) => { println!("{}", error ); }
            }
        }
        println!("no se encontró un archivo de subtítulos", );
        Ok(())
    }

    let re_season_episode = Regex::new(r".*([sS][0-9][0-9][eE][0-9][0-9]).*").unwrap();
    let re_vid_ext = Regex::new(r".*(mkv|avi|mp4)$").unwrap();
    let re_srt = Regex::new(r".*srt$").unwrap();
    let paths: fs::ReadDir = fs::read_dir("./").unwrap();

    for path in paths {
        match path {
            Ok(path) => {
                println!(" - path actual: {:?}", path.file_name());

                if re_vid_ext.is_match(&path.file_name().to_str().unwrap()) {

                    let season_and_episode = RegexBuilder::new(re_season_episode.captures(&path.file_name().to_str().unwrap())
                                                               .unwrap()
                                                               .get(1)
                                                               .map_or("", |m| m.as_str())
                    ).case_insensitive(true).build().unwrap();

                    find_srt_and_rename(path.path(), &re_srt, &season_and_episode).unwrap();
                }
            },
            Err(e) => println!("{:?}", e)
        }
    }
}
