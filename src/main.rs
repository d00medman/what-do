extern crate rand;

use rand::thread_rng;
use rand::Rng;
use std::fs::{File, OpenOptions};
use std::path::PathBuf;
use chrono;
use std::path::Path;
use std::env;
use std::io::{BufRead, BufReader, BufWriter, Write};
use clap::{Arg, App};


fn main() {
    let matches = App::new("Whatdo")
        .about("Fixing decision fatigue through randomness")
        .arg(Arg::with_name("new-opt")
            .short("nop")
            .long("new-opt")
            .takes_value(true)
            .help("Add a new option"))
        .get_matches();

    let nopt = matches.value_of("new-opt");
    match nopt {
        None => whatdo(),
        Some(s) => {
            let write_input = format!("{}{}", s, "\n");
            write_to_list("opts.txt",&write_input)
        }
    }
}



fn whatdo() {

    let opt_filename = get_target_path("opts.txt");
    let prio_filename = get_target_path("prio.txt");

    let opts = read_from_file(opt_filename);
    let prios = read_from_file(prio_filename);
    
    let mut rng = thread_rng();
    let mut roll = rng.gen_range(0..opts.len() * 2);
    let now = chrono::offset::Local::now();

    if roll > opts.len() {
        roll = rng.gen_range(0..prios.len());
        println!("{} {}", now, prios[roll])
    } else {
        println!("{} {}", now, opts[roll])
    }
}

fn get_target_path(file_name: &str) -> PathBuf {
    let home = env::var("HOME").unwrap_or("none".to_string());
    let base_path = Path::new(&home).join("codebase/scripts/what-do/src/");

    base_path.join(file_name)
}

fn read_from_file(file_name: PathBuf) -> Vec<String> {
    let file = File::open(file_name).unwrap();
    let reader = BufReader::new(file);

    let mut out = Vec::new();
    for (_, line) in reader.lines().enumerate() {
        let line = line.unwrap(); // Ignore errors.
        out.push(line);
    }
    out
}

fn write_to_list(file_name: &str, to_write: &str) {
    let target_file = get_target_path(file_name);
    println!("target file {:?}", target_file);
    let mut file;// = OpenOptions::new().write(true).open(target_file);
    match  OpenOptions::new().write(true).open(target_file) {
        Ok(f) => file = f,
    
        Err(e) => { 
            println!("Error opening target file: {}", e);
            return;
        }
    }
    match file.write(to_write.as_bytes()) {
        Ok(_) => println!("file written"),
        Err(e) => println!("Error in writing file: {}", e)
    }
}