use clap::{ArgGroup, CommandFactory, ErrorKind, Parser};
use expand_braille::{paint, BrailleGlyph};

use std::{
    fs::File,
    io::{self, Error, Read, Write},
    string::FromUtf8Error,
};

#[derive(Debug)]
pub enum MyError {
    FileIo(Error),
    Utf8Error(FromUtf8Error),
    OutsideBrailleBlock,
}

impl From<Error> for MyError {
    fn from(e: Error) -> Self {
        MyError::FileIo(e)
    }
}

impl From<FromUtf8Error> for MyError {
    fn from(e: FromUtf8Error) -> Self {
        MyError::Utf8Error(e)
    }
}

impl From<()> for MyError {
    fn from(_: ()) -> Self {
        MyError::OutsideBrailleBlock
    }
}

#[derive(Parser, Debug)]
#[clap(version, long_about = None)]
struct Args {
    /// Input location, stdin by default
    infile: Option<String>,

    #[clap(short, long)]
    /// Output location, stdout by default
    outfile: Option<String>,

    #[clap(short, long, default_value_t = '█')]
    replace: char,

    #[clap(short, long)]
    /// Braille glyph to expand
    glyph: Option<char>,
}

fn main() -> Result<(), MyError> {
    let args: Args = Args::parse();

    if let Some(glyph) = args.glyph {
        if args.infile.is_some() || args.outfile.is_some() {
            let mut arg = Args::command();
            arg.error(
                ErrorKind::ArgumentConflict,
                "Can't use both glyph and input/output file",
            )
            .exit();
        }

        let glyph = BrailleGlyph::try_from(glyph)?;
        println!("{}", glyph.paint(args.replace));

        return Ok(());
    }

    let mut reader: Box<dyn Read> = if let Some(name) = args.infile {
        Box::new(File::open(name)?)
    } else {
        Box::new(io::stdin())
    };

    let mut writer: Box<dyn Write> = if let Some(name) = args.outfile {
        Box::new(File::create(name)?)
    } else {
        Box::new(io::stdout())
    };

    let mut input = Vec::new();
    reader.read_to_end(&mut input)?;
    let input = String::from_utf8(input)?;
    let output = paint(&input, args.replace);
    writer.write_all(output.as_bytes())?;

    Ok(())
}
