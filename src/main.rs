use std::env;

const HELP: &str = r#"              
              )
           ( /(
 `  )   (  )\())
 /(/(   )\(_))/
((_)_\ ((_) |_ 
| '_ \) _ \  _|
| .__/\___/\__|
|_|v0.1.0

> process organization tools

[commands]
    init, i        Initialize container defined in a .toml file
"#;

fn main() -> Result<(), std::io::Error> {
    let cmd = env::args().nth(1);

    if let Some(cmd) = cmd {
        match cmd.as_str() {
            "init" | "i" => pot::cmd::Cmd::init()?,
            _ => println!("{HELP}"),
        }
    } else {
        println!("{HELP}")
    }

    Ok(())
}
