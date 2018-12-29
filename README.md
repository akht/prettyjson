# prettyjson

A Tiny Command Line Tool In Rust To Make Json Pretty

## Install

If you use `cargo`,

```bash
$ git clone https://github.com/akht/prettyjson.git
$ cd prettyjson
$ cargo install --path .
```

## Usage

```bash
$ prettyjson '{"foo":1,"bar":2}'
```

or

```bash
$ echo '{"foo":1,"bar":2}' | prettyjson
```

output

```bash
{
  "bar": 2,
  "foo": 1
}
```

**That's all!**
