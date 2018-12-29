extern crate serde_json;

use std::io::{self, Read};
use failure::Error;
use structopt::StructOpt;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(StructOpt, Debug)]
struct Opt {
    #[structopt(name = "INPUT")]
    input: String,
}


fn main() -> Result<()> {
    let opt = Opt::from_args();
    Ok(println!("{}", prettify(&opt.input)?))
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
