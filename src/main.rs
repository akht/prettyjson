extern crate serde_json;
extern crate clipboard;

use atty::Stream;
use failure::Error;
use std::io::{self, Read};
use structopt::StructOpt;
use clipboard::ClipboardProvider;
use clipboard::ClipboardContext;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(StructOpt, Debug)]
struct Opt {
    #[structopt(name = "INPUT")]
    input: Option<String>,
}

fn read_from_stdin() -> Result<String> {
    let mut buf = String::new();
    let stdin = io::stdin();
    let mut handle = stdin.lock();
    handle.read_to_string(&mut buf)?;

    Ok(buf)
}

fn is_stdin(input: Option<&String>) -> bool {
    let is_request = match input {
        Some(s) if s == "-" => true,
        _ => false,
    };

    let is_pipe = !atty::is(Stream::Stdin);

    is_request || is_pipe
}

fn main() -> Result<()> {
    let opt = Opt::from_args();

    if opt.input.is_none() && !is_stdin(opt.input.as_ref()) {
        // 引数がなければクリップボードを使う
        let mut ctx: ClipboardContext = ClipboardProvider::new().unwrap();
        let content = match ctx.get_contents() {
            Ok(s) => s,
            Err(_) => String::from(""),
        };
        ctx.set_contents(prettify(&content)?.to_owned()).unwrap();

        Ok(())
    } else {
        let input = match opt.input {
            Some(s) => s,
            None => read_from_stdin()?,
        };

        if input.is_empty() {
            Opt::clap().get_matches().usage();
        }

        Ok(println!("{}", prettify(&input)?))
    }
}

fn prettify(input: &str) -> Result<String> {
    let obj: serde_json::Value = serde_json::from_str(&input)?;
    Ok(serde_json::to_string_pretty(&obj)?.to_string())
}

#[cfg(test)]
mod tests {
    use crate::prettify;

    #[test]
    fn prettify_ok() {
        let expected = r#"{
  "bar": 2,
  "foo": 1
}"#;
        let input = r#"{"foo":1,"bar":2}"#;
        let actual = prettify(&input).unwrap();
        assert_eq!(expected, actual);
    }
}
