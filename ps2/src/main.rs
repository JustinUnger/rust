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
use std::fs::File;

use std::thread;

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

        let cmd_line = String::from(cmd_line);

        if program != "" { 
            self.cmd_history.push(cmd_line.clone());
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

    fn run_cd(&self, cmd_line: String) -> Result<(),io::Error> {
        let argv: Vec<&str> = cmd_line.split_whitespace().collect();
        env::set_current_dir(&argv[1])?;
        Ok(())
    }

    fn run_cmdline(&self, cmd_line: String, background: bool) -> Result<(),io::Error> {
        let argv: Vec<_> = cmd_line.split_whitespace().collect();
        let program = argv[0];
        if self.cmd_exists(program) {
            self.run_cmd(cmd_line.clone(),background);
            Ok(())
        } else {
            println!("{}: command not found", program);
            Ok(())
        }
    }

    fn run_cmd(&self, cmd_line: String, background: bool) {
		// child: JoinHandle<T> 

        let child = thread::spawn(move || {
            let cmd = foo(&cmd_line);
            let argv: Vec<_> = cmd.cmd.split_whitespace().collect();
            let p = argv[0];
            let mut c = Command::new(p);
            c.args(&argv[1..]);

            if let Some(fname) = cmd.outfile {
                let mut f = File::open(fname);
                if let Err(e) = f {
                    println!("Couldn't open {}, error {}", fname, e);
                    return;
                } 
            }
            if let Ok(mut child) = c.spawn() {
                if background { 
                    println!("[PID {} started]", child.id());
                }
                let _rc = child.wait();
                if background {
                    println!("[PID {} ended]", child.id());
                }
            } else {
                println!("Couldn't start {}", p);
            }
		});
        if !background {
            child.join().unwrap();
        }
    }

    fn cmd_exists(&self, cmd_path: &str) -> bool {
        Command::new("which").arg(cmd_path).status().unwrap().success()
    }
}

struct Cmd<'a> {
    cmd: &'a str,
    outfile: Option<&'a str>,
    infile:  Option<&'a str>,
}

fn foo(s: &str) -> Cmd {
   let idx_in = s.find('<');
   let idx_out = s.find('>');
   
   let c = match (idx_in,idx_out) {
        (None,None)         => Cmd { cmd: s, outfile: None, infile: None},
        (Some(ii), None)    => Cmd { cmd: &s[0..ii], outfile: None, infile: Some(&s[ii+1..])},
        (None,Some(io))     => Cmd { cmd: &s[0..io], outfile: Some(&s[io+1..]), infile: None},
        (Some(ii),Some(io)) => 
            if ii < io {
                Cmd { cmd: &s[0..ii].trim(), infile: Some(&s[ii+1..io]), outfile: Some(&s[io+1..]) }
            } else {
                Cmd { cmd: &s[0..io], outfile: Some(&s[io+1..ii]), infile: Some(&s[ii+1..]) }
            }
   };
   
   match c {
       Cmd { cmd: cc, infile: ii, outfile: oo } => Cmd { cmd: cc.trim(), infile: ii.map(|s| s.trim()), outfile: oo.map(|s| s.trim()) }
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
