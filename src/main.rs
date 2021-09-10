mod tokens;
mod scanner;

use std::env;
use std::io;
use std::process; 
use std::io::Write; // Brings .flush() into scope; won't work otherwise
// use std::sync::atomic::AtomicBool;

// static HAD_ERROR: AtomicBool = AtomicBool::new(false);
static mut HAD_ERROR: bool = false;

fn main() -> Result<(), String> {
    let args: Vec<String> = env::args().skip(1).collect();
    if args.len() > 1 { 
        println!("Usage: lox [file]"); 
        process::exit(1); 
    }
    else if args.len() == 1 { 
        run_file(&args[0]); 
    }
    else { 
        run_prompt(); 
    }
    Ok(())
}

fn run_file(path: &String) -> String{ //TODO: Run file 
    let data = std::fs::read_to_string(path).unwrap();
    data
}

fn run_prompt() { //TODO: Turn line into Option to break out of repl using null 
    loop { 
        let mut line = String::new(); 
        print!("> ");
        io::stdout().flush().unwrap(); //Shows '>' 
        io::stdin().read_line(&mut line).expect("Failed to read line");
        println!("Entered {}", line);
        run(line); 
        HAD_ERROR = false;

    }

}

fn run(source: String) { 
    let scanner: Scanner = Scanner::new(source);
    let tokens: Vec<Token> = Scanner.scanTokens(); 

    for token in tokens { 
        println!{"{}", token}; 
    }
}

fn error(line: i32, message: String) {
    report(line, String::from(""), message); 
}

fn report(line: i32, location: String, message: String) {
    eprintln!("[line {} ] Error {} : {}", line, location, message); 
    HAD_ERROR = true;
}
