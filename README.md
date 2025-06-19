# `wsh`

A small shell I am writing to learn Rust.

## Features
- builtins (`cd`, `pwd`, `lsv`, etc.)
- run programs from path
- set variables: `foo=bar`
- expand variables: `echo $foo -> echo bar`
- pipes: `cat Cargo.lock | grep "name"`

## Example

Set and expand variables:
```
$ foo=bar
$ hello=world
$ echo $foo $hello
bar world
```

List all variables using `lsv`:
```
$ lsv
2 items:
hello: world
foo: bar
```

Get a the value of a variable using `get`:
```
$ get foo
bar
```

Pipe commands:
```
cat Cargo.lock | grep "name"
```
