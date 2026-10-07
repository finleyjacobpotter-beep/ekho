use std::io::{self};
use std::env;
use ekho::{self};

fn main() -> io::Result<()> {
    let mut args: Vec<String> = env::args().collect::<Vec<_>>();
    args.remove(0);
    let mut stdout = io::stdout().lock();
    let _ = ekho::parse(&mut args, &mut stdout);
    let _ = ekho::print(args, &mut stdout);
    Ok(())
}
