use std::{fs, path::PathBuf};
use structopt::StructOpt;

use wast::{
    parser::{self, ParseBuffer},
    Error, ModuleKind, Wat,
};

#[derive(StructOpt)]
pub enum Utils {
    Parse {
        #[structopt(parse(from_os_str))]
        file: PathBuf,
    },
}

fn main() -> Result<(), Error> {
    match Utils::from_args() {
        Utils::Parse { file } => {
            let input = fs::read_to_string(file.as_path()).expect("file");
            let buffer = ParseBuffer::new(&input).expect("ParseBuffer");
            let wat = parser::parse::<Wat>(&buffer).expect("parse");
            let fields = match wat.module.kind {
                ModuleKind::Text(fields) => fields,
                ModuleKind::Binary(..) => todo!(),
            };
            println!("{:#?}", fields);
        }
    }
    Ok(())
}
