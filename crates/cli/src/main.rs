use std::{io::{self, Write}, fmt::format};

fn main() -> io::Result<()> {

    loop {
        let stdin = io::stdin();
        let mut stdout = io::stdout();
        let mut stderr = io::stderr();
    
        let mut input = String::new();
        let mut env = ducky::Env::default();
    
        loop {
            write!(stdout, "🦆 ")?;
            stdout.flush()?;
    
            stdin.read_line(&mut input)?;
            match run(input.trim(), &mut env) {
                Ok(Some(val)) => {
                    writeln!(stdout, "{}", val)?;
                }
                Ok(None) => {},
                Err(msg) => {
                    writeln!(stderr, "{}", msg)?;
                    stderr.flush()?;
                }
            }
            input.clear();
        }
    }

    fn run(input: &str, env: &mut ducky::Env) -> Result<Option<ducky::Val>, String> {
        let parse = ducky::parse(input).map_err(|msg| format!("Parse error: {}", msg))?;

        let evaluated = parse
                .eval(env)
                .map_err(|msg| format!("Evaluation error: {}", msg))?;

        if evaluated == ducky::Val::Unit {
            Ok(None)
        } else {
            Ok(Some(evaluated))
        }
    }
}