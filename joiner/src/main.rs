use std::env;
use std::fs::File;
use std::io::Read;
use std::io::{self, Write};


fn main() {
    let mut args = env::args();
    if args.len() != 3 {
        println!("usage: {} <msgfile> <keyfile>", args.nth(0).unwrap());
        return;
    }

    let args = args.collect::<Vec<_>>();

    let fn1 = &args[1];
    let fn2 = &args[2];

    let mut msg_file = File::open(fn1).expect("can't open msg file");
    let mut key_file = File::open(fn2).expect("can't open key file");

    let mut msg_bytes = vec![];
    let mut key_bytes = vec![];

    msg_file.read_to_end(&mut msg_bytes).expect("couldn't read msg");
    key_file.read_to_end(&mut key_bytes).expect("couldn't read key");

    for i in 0..msg_bytes.len() {
        msg_bytes[i] = msg_bytes[i] ^ key_bytes[i];
    }

    let msg_bytes = msg_bytes;

    io::stdout().write(&msg_bytes);
}
