use std::fs::File;
use std::io::BufReader;

use argh::FromArgs;

#[derive(FromArgs, Debug)]
/// An optimizing brainfuck compiler. 
struct Args {
    /// input filename
    #[argh(positional, arg_name = "input")]
    input: String,

    /// output filename
    #[argh(option, short = 'o', default = "String::from(\"a.out\")")]
    output: String,
}

fn main() {
    let args: Args = argh::from_env();
    let input_file = std::fs::File::open(args.input);
}

struct BFTree<'b> {
    tokens: BumpVec<'b, BFBlock<'b>>,
}

enum BFBlock<'b> {
    Basic(BumpVec<'b, BFToken>),
    Loop(BFTree<'b>)
}

enum BFToken {
    Plus,
    Minus,
    Left,
    Right,
    Read,
    Write,
}
