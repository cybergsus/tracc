#![feature(try_trait_v2)]
use std::error::Error;
use structopt::StructOpt;
use tracc::ast::Program;
use tracc::codegen::Compile;
use tracc::error::SourceMetadata;
use tracc::grammar::Parser;

// TODO(#3): structured formatting lib (error,warning,note,help, etc)
// TODO(#4): create test crate

fn main() {
    if let Err(ref e) = run() {
        eprintln!("{}", e);
        std::process::exit(1);
    }
}

fn run() -> Result<(), Box<dyn Error>> {
    use std::fs;
    use std::io::Write;
    let opt = Opt::from_args();
    let filename = opt.file;
    let file = fs::read_to_string(&filename)?;
    let out_file = opt.output.unwrap_or_else(|| filename.with_extension("s"));
    let meta = SourceMetadata::new(&file).with_file(filename);
    let program: Program = Parser::new(&meta).parse()?;
    let output = program.compile();
    let mut file = fs::File::create(out_file)?;
    for x in output {
        writeln!(file, "{}", x)?;
    }

    Ok(())
}

#[derive(Debug, StructOpt)]
struct Opt {
    /// The file to compile
    #[structopt(parse(from_os_str))]
    file: std::path::PathBuf,
    /// The (optional) output file
    #[structopt(short = "o", long = "output", parse(from_os_str))]
    output: Option<std::path::PathBuf>,
}
