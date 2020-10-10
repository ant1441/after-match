use std::fs;
use std::io::{self, prelude::*, BufReader};

use anyhow::Result;
use regex::Regex;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt()]
struct Opt {
    pattern: String,
    /// Input file, '-' or missing will use stdin
    path: Option<String>,
}

enum Input {
    Standard(io::Stdin),
    File(fs::File),
}

impl io::Read for Input {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        match self {
            Input::Standard(s) => s.read(buf),
            Input::File(f) => f.read(buf),
        }
    }
}

fn main() -> Result<()> {
    let opt = Opt::from_args();
    let path = opt.path.unwrap_or_else(|| "-".to_string());
    let pattern = Regex::new(&opt.pattern)?;

    let input = {
        if path == "-" {
            let s = io::stdin();
            BufReader::new(Input::Standard(s))
        } else {
            let f = fs::File::open(path)?;
            BufReader::new(Input::File(f))
        }
    };

    let mut matched = false;

    for line in input.lines() {
        let line = line?;
        if pattern.is_match(&line) {
            matched = true;
        }
        if matched {
            println!("{}", line);
        }
    }

    Ok(())
}
