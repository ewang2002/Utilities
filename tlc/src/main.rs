use clap::{AppSettings, Clap};
use std::fs::{File};
use std::error::Error;
use std::path::{Path, PathBuf};
use std::io::{BufReader, BufRead};
use std::ffi::OsStr;
use std::collections::VecDeque;

fn main() -> Result<(), Box<dyn Error>> {
    let opts: Opts = Opts::parse();

    let dir = Path::new(&opts.dir);
    let dirs_to_avoid = opts.dirs_to_ignore
        .split_ascii_whitespace()
        .map(|x| OsStr::new(x))
        .collect::<Vec<_>>();
    let ext_to_check = opts.extensions_to_check
        .split_ascii_whitespace()
        .map(|x| OsStr::new(x))
        .collect::<Vec<_>>();

    let mut line_ct: usize = 0;
    let mut dir_queue: VecDeque<PathBuf> = VecDeque::new();
    dir_queue.push_back(PathBuf::from(dir));

    while !dir_queue.is_empty() {
        let this_dir = dir_queue.pop_front().unwrap();
        let directory = match this_dir.read_dir() {
            Ok(d) => d,
            Err(_) => continue
        };

        for f in directory {
            let file = f.unwrap();
            let entry = file.path();
            let name = file.file_name();

            if entry.is_dir() {
                if dirs_to_avoid.iter().any(|x| (*x) == name) {
                    continue;
                }

                dir_queue.push_back(entry);
                continue;
            }

            match entry.extension() {
                None => continue,
                Some(extension) => {
                    if !ext_to_check.contains(&extension) {
                        continue;
                    }
                }
            }


            let file = match File::open(entry.as_path()) {
                Ok(f) => f,
                Err(_) => continue
            };

            let ct = BufReader::new(file).lines().count();
            println!("{0:<10} {1}", ct, entry.as_path().display());
            line_ct += ct;
        }
    }

    println!("Total Lines: {}", line_ct);
    return Ok(());
}

#[derive(Clap)]
#[clap(version = "0.1.0", about = "Recursively gets the total line count of all files in one or more directories.")]
#[clap(setting = AppSettings::ColoredHelp)]
struct Opts {
    #[clap(short('d'), long("dir"), default_value = ".")]
    dir: String,

    #[clap(short('i'), long("ignore"), default_value = "")]
    dirs_to_ignore: String,

    #[clap(short('c'), long("check"), default_value = "")]
    extensions_to_check: String,
}


// Recursive solution:
// fn get_line_ct(path: &Path, dirs_avoid: &Vec<&OsStr>, ext_check: &Vec<&OsStr>) -> usize {
//     let mut line_ct: usize = 0;
//     let directory = match path.read_dir() {
//         Ok(d) => d,
//         Err(_) => return line_ct
//     };
//
//     for f in directory {
//         let file = f.unwrap();
//         let entry = file.path();
//         let name = file.file_name();
//
//         if entry.is_dir() {
//             if dirs_avoid.iter().any(|x| (*x) == name) {
//                 continue;
//             }
//
//             line_ct += get_line_ct(entry.as_path(), dirs_avoid, ext_check);
//             continue;
//         }
//
//
//         match entry.extension() {
//             None => continue,
//             Some(extension) => {
//                 if !ext_check.contains(&extension) {
//                     continue;
//                 }
//             }
//         }
//
//
//         let file = match File::open(entry.as_path()) {
//             Ok(f) => f,
//             Err(_) => continue
//         };
//
//         let ct = BufReader::new(file).lines().count();
//         println!("{0:<10} {1}", ct, entry.as_path().display());
//         line_ct += ct;
//     }
//
//     return line_ct;
// }