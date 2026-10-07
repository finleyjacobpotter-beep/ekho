use std::io::{Write, StdoutLock, Error};

const FLAGS: [&str; 1] = [
    "-h"
];


/// First line is a short summary describing function.
///
/// The next lines present detailed documentation. Code blocks start with
/// triple backquotes and have implicit `fn main()` inside
/// and `extern crate <cratename>`. Assume we're testing a `playground` library
/// crate or using the Playground's Test action:
///
/// ```
/// assert_eq!(1, 1);
/// ```
fn help(stdout: &mut StdoutLock) -> Result<(), Error> {
    let _ = stdout.write_all(b"usage\n");
    let _ = stdout.write_all(b"---------\n");
    let _ = stdout.write_all(b" -h | show this help message\n");
    let _ = stdout.write_all(b" -n | don't print newline\n");
    Ok(())
}

// print help and exit if -h is first arg
// otherwise print all args
pub fn parse(args: &mut Vec<String>, stdout: &mut StdoutLock) -> Result<Vec<String>, Error> {
    while !args.is_empty() {
        if FLAGS.contains(&args[0].as_str()) {
            if args[0].as_str() == FLAGS[0] {
                let _ = help(stdout);
                break;
            }
            args.remove(0);
        } else {
            break;
        }
    }
    Ok((args).to_vec())
}


pub fn print(args: Vec<String>, stdout: &mut StdoutLock) -> Result<(), Error> {
    let mut iter = args.into_iter().peekable();
    while let Some(arg) = iter.next() {
        let _ = stdout.write_all(arg.as_bytes());
        if iter.peek().is_some() {
            let _ = stdout.write_all(b" ");
        }
    }
    let _ = stdout.write_all(b"\n");
    Ok(())
}


#[cfg(test)]
mod tests {
    #[test]
    fn test_print() {
        assert_eq!(3, 3);
    }

}
