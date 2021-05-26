use std::fs::{File, read_dir, metadata};
use std::io::{prelude::*, BufReader};
use std::process::exit;
use std::path::Path;
use std::cmp::Reverse;
use std::convert::TryFrom;
use clap::{App, Arg};

fn linecount(f: &str) -> Result<u32, std::io::Error> {
    // Count number of lines in file
    let file = File::open(f);
    let file = match file {
        Ok(f) => f,
        Err(e) => return Err(e),
    };
    let reader = BufReader::new(file);

    // Avoid for loop with this...
    let filelines = u32::try_from(reader.lines().count()).unwrap();

    Ok(filelines)
}

fn filenames(d: &str) -> Result<Vec<String>, std::io::Error> {
    // Get files in dir as strings,
    // return a Vec with filenames
    let mut fnames = Vec::new();
    let filepaths = read_dir(d);
    let filepaths = match filepaths {
        Ok(f) => f,
        Err(e) => return Err(e),
    };
    for p in filepaths {
        // Convert Pathbuf to str
        let pathname = p.unwrap().path();
        let path_as_str = pathname.to_str().unwrap();
        if metadata(path_as_str).unwrap().is_file() {
            fnames.push(path_as_str.to_string());
        }
    }

    Ok(fnames)
}

fn main() {
    let matches = App::new("lc")
                          .version("1.0")
                          .author("Magnus W. <magnuswallin@tutanota.com>")
                          .about("Counts lines in files. Exciting!")
                          .arg(Arg::with_name("TARGETDIR")
                              .help("Look for files in this directory")
                              .required(true)
                              .index(1))
                          .arg(Arg::with_name("descend")
                              .help("Sort descending by linecount")
                              .short("d"))
                          .get_matches();

    // Sanity checks
    let directory = matches.value_of("TARGETDIR").unwrap();
    if ! Path::new(directory).exists() {
        eprintln!("ERROR: Directory '{}' does not exist. Exiting.", directory);
        exit(1);
    }
    if ! metadata(directory).unwrap().is_dir() {
        eprintln!("ERROR: '{}' is not a directory. Exiting.", directory);
        exit(1);
    }
    if let Err(e) = filenames(directory) {
        eprintln!("ERROR: Could not open directory '{}':\n{}", directory, e);
        exit(1);
    }

    let mut reverse: bool = false;
    // Sort ascending by default, unless '-d' flag
    if matches.is_present("descend") {
        reverse = true;
    }

    struct Filelines {
        name: String,
        lines: u32,
    }

    // Save structs here
    let mut filesvec = Vec::new();
    // Get filenames
    let fnamevec = filenames(directory).unwrap();
    if fnamevec.is_empty() {
        println!("No files found in directory '{}'. Exiting.", directory);
        exit(0);
    }

    for f in fnamevec {
        // Don't panic if we can't open a file
        if let Err(e) = linecount(&f) {
            eprintln!("WARN! Could not open file: {}:\n{}", f, e);
            continue;
        }
        // Only save filename, not full path, in struct
        let fullpath: Vec<_> = f.split('/').collect();
        let basename = fullpath[fullpath.len()-1];
        filesvec.push(
            Filelines {
                name: basename.to_string(),
                lines: linecount(&f).unwrap(),
            }
        )
    }

    println!("{}:\n{:<50} {:<5}", directory, "Filename", "Lines");
    println!("{:-<56}", "");
    // Sort vec by number of lines and print to stdout
    if reverse {
        filesvec.sort_by_key(|x| Reverse(x.lines));
        for i in filesvec {
            println!("{:<50} {:<5}", i.name, i.lines);
        }
    } else {
        filesvec.sort_by_key(|x| x.lines);
        for i in filesvec {
            println!("{:<50} {:<5}", i.name, i.lines);
        }
    }
}
