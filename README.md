# `wsh`

A small shell I am writing to learn Rust.

## Features
- builtins (`cd`, `pwd`, `lsv`, etc.)
- run programs from path
- set variables: `foo=bar`
- expand variables: `echo $foo -> echo bar`
- pipes: `cat Cargo.lock | grep "name"`
- input/output redirection: `echo "hello world" > msg.txt`

## Configuration

The configuration file is located at `$XDG_CONFIG_HOME/wsh/config.toml`.
If `XDG_CONFIG_HOME` is not set, `$HOME/.config` is used instead.

### Example
```toml
# ~/.config/wsh/config.toml

prompt = "> "
```

## Usage

Set and expand variables:
```
$ foo=bar
$ hello=world
$ echo $foo $hello
bar world
```

Pipe commands:
```
cat Cargo.lock | grep "name"
```

Redirect IO:
```
cat < input.txt | grep "foo" | wc -l > count.txt
```

Run `help` command for more details.
