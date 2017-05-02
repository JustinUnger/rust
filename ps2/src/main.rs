//
// gash.rs
//
// Starting code for PS2
// Running on Rust 1+
//
// University of Virginia - cs4414 Spring 2014
// Weilin Xu, David Evans
// Version 0.4
//

extern crate getopts;

use getopts::Options;
use std::env;
use std::io::{self, Write};
use std::process::Command;
use std::error::Error;

struct Shell<'a> {
    cmd_prompt: &'a str,
    cmd_history: Vec<String>
}

impl <'a>Shell<'a> {
    fn new(prompt_str: &'a str) -> Shell<'a> {
        Shell { cmd_prompt: prompt_str, cmd_history: vec![] }
    }

    fn run(&mut self) {
        let stdin = io::stdin();
        let mut stdout = io::stdout();

        loop {
            stdout.write(self.cmd_prompt.as_bytes()).unwrap();
            stdout.flush().unwrap();

            let mut line = String::new();

            stdin.read_line(&mut line).unwrap();
            let cmd_line = line.trim();
            match self.process_cmdline(cmd_line) {
                Ok(_) => continue,
                Err(e) => io::stdout().write(e.description().as_bytes()).unwrap()
            };
        }
    }

    fn process_cmdline(&mut self, mut cmd_line: &str) -> Result<(),io::Error> {
        let program = cmd_line.splitn(2, ' ').nth(0).expect("no program");
        let cmd_line_chars: Vec<char> = cmd_line.chars().collect();
        let len = cmd_line_chars.len();
        let background = if len == 0 { false } else { cmd_line_chars[len-1] == '&'};

        if background {
            cmd_line = &cmd_line[0..len-1];
        }

        let cmd_line = cmd_line;

        if program != "" { 
            self.cmd_history.push(String::from(cmd_line));
        }

        match program {
            ""      =>  { Ok(()) }
            "exit"  =>  { Ok(()) }
            "cd"    =>  { self.run_cd(cmd_line) }
            "history" => { Ok(self.run_history()) }
            _       =>  { self.run_cmdline(cmd_line,background) }
        }
    }

    fn run_history(&self) {
        for (i,cmd) in self.cmd_history.iter().enumerate() {
            io::stdout().write(format!("  {}   {}\n",i,cmd).as_bytes()).unwrap();
        }
    }

    fn run_cd(&self, cmd_line: &str) -> Result<(),io::Error> {
        let argv: Vec<&str> = cmd_line.split_whitespace().collect();
        env::set_current_dir(&argv[1])?;
        Ok(())
    }

    fn run_cmdline(&self, cmd_line: &str, background: bool) -> Result<(),io::Error> {
        let argv: Vec<&str> = cmd_line.split(' ').filter_map(|x| {
            if x == "" {
                None
            } else {
                Some(x)
            }
        }).collect();

        match argv.first() {
            Some(&program) => self.run_cmd(program, &argv[1..], background),
            None => Ok(())
        }
    }

    fn run_cmd(&self, program: &str, argv: &[&str], background: bool) -> Result<(),io::Error> {
        if self.cmd_exists(program) {
            if !background {
                io::stdout().write(&Command::new(program).args(argv).output().unwrap().stdout)?;
                Ok(())
            } else {
                let mut c = Command::new(program);
                let c = c.args(argv);
                match c.spawn() {
                    Ok(child) => io::stdout().write(format!("[PID {}]\n",child.id()).as_bytes()),
                    Err(e) => Err(e)
                };
                Ok(())
            }
        } else {
            println!("{}: command not found", program);
            Ok(())
        }
    }

    fn cmd_exists(&self, cmd_path: &str) -> bool {
        Command::new("which").arg(cmd_path).status().unwrap().success()
    }
}

fn get_cmdline_from_args() -> Option<String> {
    /* Begin processing program arguments and initiate the parameters. */
    let args: Vec<_> = env::args().collect();

    let mut opts = Options::new();
    opts.optopt("c", "", "", "");

    opts.parse(&args[1..]).unwrap().opt_str("c")
}

fn main() {
    let opt_cmd_line = get_cmdline_from_args();

    match opt_cmd_line {
        //Some(cmd_line) => Shell::new("").run_cmdline(&cmd_line, false),
        Some(_)        => Shell::new("gash > ").run(),
        None           => Shell::new("gash > ").run(),
    }
}
