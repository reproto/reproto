use std::io;
use std::io::Write as _;
use std::io::BufRead;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let stdin = io::stdin();
    let stdout = io::stdout();

    let input = stdin.lock();
    let mut stdout = stdout.lock();

    for line in input.lines() {
        let line = line?;
        let entry: reproto_it::generated::test::Entry = serde_json::from_str(&line)?;
        write!(stdout, "#<>{}\n", serde_json::to_string(&entry)?)?;
        stdout.flush()?;
    }

    Ok(())
}