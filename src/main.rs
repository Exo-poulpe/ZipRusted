use std::{
    env,
    fs::{self, File},
    io::{BufRead, BufReader},
    ptr::null,
};

use zip::{read, ZipArchive};

use colored::Colorize;

use clap::Parser;

/// Simple monothread ZIP bruteforcer in Rust
#[derive(Parser, Debug)]
#[command(about, long_about = None,arg_required_else_help(true))]
struct Args {
    /// Filepath of the archive to bruteforce
    #[arg(short, long)]
    file: String,

    /// Wordlist filepath
    #[arg(short, long)]
    wordlist: String,

    /// Enable verbose mode
    #[arg(short, long, default_value_t = false)]
    verbose: bool,
}

fn main() {
    let args = Args::parse();


    let fname = std::path::Path::new(&args.file);
    let file = match fs::File::open(fname) {
        Ok(f) => f,
        Err(e) => panic!("Error in file reading : {e}"),
    };

    let wordlist = &args.wordlist;

    let mut crypted = false;

    let mut archive = zip::ZipArchive::new(file).unwrap();

    match archive.by_index(0) {
        Ok(_) => {
            crypted = false;
        }
        Err(_) => {
            crypted = true;
        }
    };

    if crypted {
        encrypted_archive(&mut archive, wordlist, args.verbose);
    } else {
        unencrypted_archive(&mut archive);
    }
}

fn unencrypted_archive(archive: &mut ZipArchive<File>) {
    // Decompress the archive
    println!("Unencrypted archive detected\n");
    for i in 0..archive.len() {
        let mut zippedfile = match archive.by_index(i) {
            Ok(zf) => zf,
            Err(e) => panic!("Error during reading file"),
        };

        let name = zippedfile.name();
        println!("Filename : {name}");
    }
}

fn encrypted_archive(archive: &mut ZipArchive<File>, wordlist: &str, verbose: bool) {
    // Brute force with wordlist the archive
    println!("Encrypted archive detected\n");

    let mut count: u64 = 0;
    let fwords = File::open(wordlist).expect("Wordlist not found");
    let reader = BufReader::new(fwords);

    for line in reader.lines() {
        let pass = match line {
            Ok(line) => line,
            Err(_) => {
                // Invalid UTF-8 encountered, skip this line
                if verbose {
                    println!("Encountered invalid UTF-8, skipping line.");
                }
                continue;
            }
        };

        let attempt = {
            match archive.by_index_decrypt(0, pass.as_bytes()) {
                Ok(zf) => true,
                Err(_) => false,
            }
        };

        if attempt {
            if verbose {
                println!("[{}] Retest with the password : \"{}\" on the index : 1","*".blue(),pass);
                // println!("{} {} {}","Retest with password :".green(),pass, "of index : 1".green());
            }
            match archive.by_index_decrypt(1, pass.as_bytes()) {
                Err(e) => continue,
                Ok(f) => f,
            };
            println!("Success with password : \"{pass}\"");
            break;
        } else {
            if verbose {
                println!("Password : \"{0}\" Failed", pass); 
            }
        }

        count += 1;
    }

    println!("Number of password tested : {count}");
}
