use std::io::{self, Write, StdoutLock, Error};
use std::env;


const FLAGS: [&str; 1] = [
    "-h"
];

// print help message
fn help(stdout: &mut StdoutLock) -> Result<(), Error> {
    let _ = stdout.write_all(b"usage\n");
    let _ = stdout.write_all(b"---------\n");
    let _ = stdout.write_all(b" -h | show this help message\n");
    let _ = stdout.write_all(b" -n | don't print newline\n");
    Ok(())
}

// print help and exit if -h is first arg
// otherwise print all args
fn parse(args: &mut Vec<String>, stdout: &mut StdoutLock) -> Result<Vec<String>, Error> {
    while args.len() > 0 {
        if FLAGS.contains(&args[0].as_str()) {
            if &args[0].as_str() == &FLAGS[0] {
                let _ = help(stdout);
                break;
            }
            args.remove(0);
        } else {
            break;
        }
    }
    return Ok((args).to_vec());
}


fn print(args: Vec<String>, stdout: &mut StdoutLock) -> Result<(), Error> {
    let mut iter = args.into_iter().peekable();
    while let Some(arg) = iter.next() {
        let _ = stdout.write_all(&arg.as_bytes());
        if iter.peek().is_some() {
            let _ = stdout.write_all(b" ");
        }
    }
    let _ = stdout.write_all(b"\n");
    Ok(())
}

fn main() -> io::Result<()> {
    let mut args: Vec<String> = env::args().collect::<Vec<_>>();
    args.remove(0);
    let mut stdout = io::stdout().lock();
    let _ = parse(&mut args, &mut stdout);
    let _ = print(args, &mut stdout);
    Ok(())
}
