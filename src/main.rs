use std::env;
use std::io::Write;

use anyhow::Context;
use anyhow::Result;

fn main() -> Result<()> {
    let mut args = env::args();

    match (args.next(), args.next(), args.next()) {
        (Some(_), None, _) => run_prompt(),
        (Some(_), Some(file), None) => run_file(file),
        (_, _, Some(_)) => anyhow::bail!("Wrong number of arguments"),
        _ => panic!("impossible"),
    }
}

fn run_prompt() -> Result<()> {
    while let Some(line) = prompt()? {
        run(line)?;
    }

    println!("\nbybye");

    Ok(())
}

fn prompt() -> Result<Option<String>> {
    let mut buff = String::new();

    print!("\n> ");

    std::io::stdout()
        .flush()
        .context("Fatal error printing prompt")?;

    let bytes = std::io::stdin()
        .read_line(&mut buff)
        .context("Fatal error reading lines")?;

    if bytes == 0 {
        Ok(None)
    } else {
        Ok(Some(buff))
    }
}

fn run_file(file: String) -> Result<()> {
    let content = std::fs::read_to_string(file).context("Fatal error reading file")?;

    run(content)?;

    Ok(())
}

fn run(code: String) -> Result<()> {
    // Add scanner and token list here
    println!("\nExecuting code: {}", code);

    Ok(())
}
