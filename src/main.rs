use std::{
    env,
    io::{stdin, stdout, Write},
    path::Path,
    process::{Command, Stdio, Child},
};

fn handle_cd(args: std::str::SplitWhitespace<'_>) {
    let new_dir = args.peekable().peek().map_or("/", |x| *x);
    let root = Path::new(new_dir);

    if let Err(e) = env::set_current_dir(root) {
        eprintln!("{}", e);
    }
}

fn main() {
    loop {
        print!("{}$ ", env::current_dir().unwrap().to_str().unwrap());
        let _ = stdout().flush();
        let mut input = String::new();

        stdin().read_line(&mut input).unwrap();

        let mut commands = input.trim().split(" | ").peekable();
        let mut previous_command = None;

        while let Some(command) = commands.next() {
            let mut parts = command.trim().split_whitespace();
            let program = parts.next().unwrap();
            let args: std::str::SplitWhitespace<'_> = parts;

            match program {
                "cd" => {
                    handle_cd(args);
                }

                "exit" => return,

                program => {
                    let stdin = previous_command.map_or(
                        Stdio::inherit(), 
                        |output: Child| Stdio::from(output.stdout.unwrap())
                    );

                    let stdout = if commands.peek().is_some() {
                        Stdio::piped()
                    } else {
                        Stdio::inherit()
                    };

                    let output = Command::new(program).args(args).stdin(stdin).stdout(stdout).spawn();

                    match output {
                        Ok(output) => {previous_command = Some(output)},
                        Err(e) => {
                            previous_command = None;
                            eprintln!("{}", e)
                        },
                    };
                }
            }
        }

        if let Some(mut final_command) = previous_command {
            if let Err(e) = final_command.wait() {
                eprintln!("{}", e);
            }
        }
    }
}
