use argh::FromArgs;

mod block;

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
    let input = std::fs::read(args.input).unwrap();

    println!(
        "{:#?}",
        BFTree::from_byte_iter(&mut input.into_iter().chain(Some(b']'))),
    );
}

#[derive(Debug)]
struct BFTree {
    blocks: Vec<BFBlock>,
}

#[derive(Debug)]
enum BFBlock {
    Basic(Vec<BFToken>),
    Loop(Vec<BFBlock>),
}

#[derive(Debug)]
enum BFToken {
    Plus,
    Minus,
    Left,
    Right,
    Read,
    Write,
}

impl BFTree {
    fn from_byte_iter(iter: &mut impl Iterator<Item = u8>) -> Option<Self> {
        let mut blocks = Vec::new();
        let mut cur_block = Vec::new();

        while let Some(b) = iter.next() {
            match b {
                b'[' => {
                    if !cur_block.is_empty() {
                        blocks.push(BFBlock::Basic(cur_block));
                        cur_block = Vec::new();
                    }
                    blocks.push(BFBlock::Loop(BFTree::from_byte_iter(iter)?.blocks))
                }
                b']' => {
                    if !cur_block.is_empty() {
                        blocks.push(BFBlock::Basic(cur_block));
                    }
                    return Some(Self { blocks });
                }
                _ => cur_block.extend(BFToken::from_byte(b)),
            }
        }

        None
    }
}

impl BFToken {
    fn from_byte(b: u8) -> Option<Self> {
        match b {
            b'+' => Some(Self::Plus),
            b'-' => Some(Self::Minus),
            b'<' => Some(Self::Left),
            b'>' => Some(Self::Right),
            b',' => Some(Self::Read),
            b'.' => Some(Self::Write),
            _ => None,
        }
    }
}
