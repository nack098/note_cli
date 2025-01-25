mod multiple_arg;
mod single_arg;

use multiple_arg::{multiple_arg_cmd, MultipleArg};
use single_arg::{single_arg_cmd, SingleArg};

#[derive(Clone, Debug, Copy)]
enum Op {
    Single(SingleArg),
    Multiple(MultipleArg),
}

impl Op {
    const VALUE_LOOKUP: [(&str, Self); 5] = [
        ("help", Self::Single(SingleArg::Help)),
        ("list", Self::Single(SingleArg::List)),
        ("create", Self::Single(SingleArg::Create)),
        ("delete", Self::Multiple(MultipleArg::Delete)),
        ("read", Self::Multiple(MultipleArg::Read)),
    ];

    fn from_string(input: &str) -> Self {
        let val = Self::VALUE_LOOKUP
            .iter()
            .find(|(name, _)| name == &input)
            .map(|(_, res)| *res);
        if let Some(val) = val {
            val
        } else {
            panic!(
                "
Argument Error
Unknown command {}
To list out all availible commands
use:
    note_cli help
                ",
                input
            );
        }
    }
}

pub fn run(args: Vec<String>, path: std::path::PathBuf) -> Result<(), std::io::Error> {
    let op = Op::from_string(args[0].as_str());

    match std::fs::create_dir_all(&path) {
        Ok(_) => (),
        Err(err) => return Err(err),
    }

    match op {
        Op::Single(op) => {
            if args.len() > 1 {
                panic!(
                    "
Argument Error
This function does not accept any arguments
use:
    note_cli {}
                    ",
                    args[0]
                )
            }
            single_arg_cmd(op, path)
        }
        Op::Multiple(op) => {
            let raw = args.get(1);
            if let Some(raw) = raw {
                let index: usize = match raw.parse() {
                    Ok(val) => val,
                    Err(_) => panic!(
                        "
Argument Error
Second argument should be number
use:
    note_cli {} <index>
                        ",
                        args[0]
                    ),
                };
                multiple_arg_cmd(op, path, index);
            } else {
                panic!(
                    "
Argument Error
Please insert the index
use:
    note_cli {} <index>
                    ",
                    args[0]
                );
            }
        }
    }
    Ok(())
}
