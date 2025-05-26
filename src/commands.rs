pub fn echo<'a>(mut args: impl Iterator<Item = &'a str>) {
    if let Some(first) = args.next() {
        print!("{}", first);
        for arg in args {
            print!(" {}", arg);
        }
    }
    println!();
}
