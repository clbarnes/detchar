use anyhow::Result;
use chardetng::EncodingDetector;
use clap::Parser;
use std::fs::File;
use std::io::{stdin, BufRead, BufReader, Read};
use std::path::{Path, PathBuf};

/// Guess character encodings for text files.
///
/// In regular mode, prints a line for each file:
/// the character encoding, a tab, then the given file path.
/// If the classifier is not sure, the line is prepended with a question mark "?".
///
/// In --combine mode, simply prints the character encoding of the files' combined contents (possibly prepended with "?").
#[derive(Parser, Debug)]
#[clap(version, about)]
struct Args {
    /// Any number of files to check for character encoding; use - for stdin
    file: Vec<PathBuf>,
    /// Any number of (UTF-8-encoded) files which contain newline-separated file paths to check for character encoding; use - for stdin
    #[clap(short, long)]
    from_file: Vec<PathBuf>,
    /// Guess a single character encoding for all files
    #[clap(short, long)]
    combine: bool,
}

fn read_paths<R: Read>(reader: &mut R) -> Vec<PathBuf> {
    BufReader::new(reader)
        .lines()
        .map(|line| PathBuf::from(line.unwrap()))
        .collect()
}

fn expand_file_arg(path: &Path) -> Result<Vec<PathBuf>> {
    if path == Path::new("-") {
        let mut sin = stdin();
        Ok(read_paths(&mut sin))
    } else {
        let mut f = File::open(path)?;
        Ok(read_paths(&mut f))
    }
}

fn populate_guesser(guesser: &mut EncodingDetector, path: &Path) -> Result<()> {
    let mut buf = Vec::default();
    if path == Path::new("-") {
        let mut sin = stdin();
        sin.read_to_end(&mut buf)?;
    } else {
        let mut f = File::open(path)?;
        f.read_to_end(&mut buf)?;
    }
    guesser.feed(&buf, false);
    Ok(())
}

fn expand_paths(files: Vec<PathBuf>, from_files: Vec<PathBuf>) -> Result<Vec<PathBuf>> {
    let mut out = files;

    for fpath in from_files.into_iter() {
        out.append(&mut expand_file_arg(&fpath)?)
    }
    Ok(out)
}

fn singles(files: Vec<PathBuf>) -> Result<()> {
    for fpath in files.into_iter() {
        let mut guesser = EncodingDetector::new();
        populate_guesser(&mut guesser, &fpath)?;
        print_result(&mut guesser, Some(fpath));
    }
    Ok(())
}

fn combine(files: Vec<PathBuf>) -> Result<()> {
    let mut guesser = EncodingDetector::new();
    for fpath in files.into_iter() {
        populate_guesser(&mut guesser, &fpath)?;
    }
    print_result(&mut guesser, None);
    Ok(())
}

fn print_result(guesser: &mut EncodingDetector, fpath: Option<PathBuf>) {
    let is_ascii = !guesser.feed(&[], true);
    let (encoding, is_good) = guesser.guess_assess(None, true);
    let enc_name = if is_ascii { "ASCII" } else { encoding.name() };
    let q = if is_ascii || is_good { "" } else { "?" };
    if let Some(p) = fpath {
        println!("{}{}\t{}", q, enc_name, p.to_string_lossy());
    } else {
        println!("{}{}", q, enc_name);
    }
}

fn main() -> Result<()> {
    let args = Args::parse();
    let paths = expand_paths(args.file, args.from_file)?;
    if paths.is_empty() {
        eprintln!("No paths given, nothing to do");
        return Ok(());
    }
    if args.combine {
        combine(paths)?
    } else {
        singles(paths)?
    }
    Ok(())
}
