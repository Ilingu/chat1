use std::{env, path::Path};

const HELP: &str = r#"
chat1 🐈‍

USAGE:
  chat1 f|t STRING

FLAGS:
  -h, --help            Prints help information

ARGS:
  First argument:  the document type, choose between a file [f] or text [t]
  Second argument: if doctype is [f] then enter the file path, otherwise enter the text to hash

RETURNS: The sha1 hash of the file or text

EXEMPLES:
  chat1 t "ilingu"
  chat1 f "./ilovecat.mp3"
"#;

pub enum AppArgs {
    File(String),
    Text(String),
}

impl AppArgs {
    pub fn parse() -> Self {
        let cliargs = env::args().skip(1).collect::<Vec<_>>();

        // Help has a higher priority and should be handled separately.
        if cliargs.len() != 2 {
            println!("{}", HELP);
            std::process::exit(0);
        }

        let (doctype, data) = (cliargs[0].to_owned(), cliargs[1].to_owned());
        match doctype.as_str() {
            "t" => Self::Text(data),
            "f" => {
                if !Path::new(&data).exists() {
                    eprintln!("Invalid filepath for your file");
                    std::process::exit(0);
                }
                Self::File(data)
            }
            _ => {
                eprintln!("Invalid doctype should either be 'f' or 't'");
                std::process::exit(0);
            }
        }
    }
}
