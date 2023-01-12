use std::fs::{self, File};
use std::path::Path;
use std::io::{self, Write};

use clap::{Parser as CLIParser};
use pest::Parser;

use evgen::{EV1Parser, Rule, Document};

/// Generate JSON from .ev file
#[derive(CLIParser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Input file
    #[arg(short, long)]
    path: String,

    /// Output file
    #[arg(short, long)]
    output: Option<String>,
}

fn main() {
    let args = Args::parse();

    let input_file = fs::read_to_string(args.path).expect("cannot read file");
    let mut parsed = EV1Parser::parse(Rule::document, &input_file).expect("could not parse file");
    let structured = Document::from_pair(parsed.next().unwrap()).expect("could not parse contents");
    let serialized = serde_json::to_string(&structured).expect("could not serialize data");

    let mut out_writer = match args.output {
        Some(x) => {
            let path = Path::new(&x);
            Box::new(File::create(&path).unwrap()) as Box<dyn Write>
        }
        None => Box::new(io::stdout()) as Box<dyn Write>,
    };

    writeln!(out_writer, "{}", serialized).expect("could not write to file");
}