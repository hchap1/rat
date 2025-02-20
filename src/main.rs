use regex::Regex;
use std::env::args;
use std::fs::read_dir;
use std::env::current_dir;
use std::fs::read_to_string;
use std::result::Result::Ok;

fn main() {

    // General structure: rat <filename> [-r pattern] providing regex over all items in CD matching that regex
    let args: Vec<String> = args().map(|x| x.to_string()).collect::<Vec<String>>();

    let files = match args.len() {
        0 | 1 => {
            eprintln!("No arguments supplied.");
            return;
        }
        2 => vec![args[1].clone()],
        _ => {
            let r: Regex = match Regex::new(&args[1]) {
                Ok(r) => r,
                Err(_) => {
                    eprintln!("Invalid regex: {}", args[1]);
                    return;
                }
            };

            let contents = match read_dir(current_dir().unwrap()) {
                Ok(contents) => contents,
                Err(_) => {
                    eprintln!("Could not read cwd.");
                    return;
                }
            };

            contents
                .filter_map(|x| x.ok())
                .filter(|x| x.path().is_file())
                .map(|file| file.file_name().to_string_lossy().to_string())
                .filter(|filename| r.is_match(filename))
                .collect()
        }
    };

    for file in files {
        match read_to_string(&file) {
            Ok(contents) => {
                println!("\n--- {file} ---");
                contents.lines().for_each(|line| println!("{line}"));
            }
            Err(_) => eprintln!("No such file {file}.")
        };
    }
}
