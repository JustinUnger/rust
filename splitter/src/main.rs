//use std::rand::random;

extern crate rand;

use std::env;
use std::io;
use std::fs::File;
use std::io::Read;
use std::io::Write;

use rand::Rng;

fn main() {
    let mut args: env::Args = env::args();
    if args.len() != 2 {
        println!("Usage: {} <inputfile>", args.nth(0).unwrap()); 
    } else {
        let fname = args.nth(1).unwrap();
        let msg_file = File::open(&fname);

        match msg_file {
            Ok(mut msg) => {
                let mut msg_bytes = Vec::new();
                let bytes_read: io::Result<usize> = msg.read_to_end(&mut msg_bytes);
                let msg_bytes = msg_bytes;
                let fn1 = fname.clone() + ".share1";
                let share1_file = File::create(&fn1);
                let fn2 = fname.clone() + ".share2";
                let share2_file = File::create(&fn2);
                
                match (share1_file, share2_file) {
                    (Ok(share1), Ok(share2)) => { 
                        split(&msg_bytes, share1, share2); 
                        } ,
                    (_, _) => panic!("Error opening output files!"),
                }
            } ,
            Err(e) => panic!("Error {} opening message file: {}", e, fname)
        }
    }
}

fn xor(a: &[u8], b: &[u8]) -> Vec<u8> {
    let mut ret = vec![];
    for i in 0..a.len() {
        ret.push(a[i] ^ b[i]);
    }
    ret
}

fn split(msg_bytes: &[u8], mut share1: File, mut share2: File) {
    let mut random_bytes: Vec<u8> = vec![];
    let mut rng = rand::thread_rng();
    // This is not cryptographically strong randomness! 
    // (For entertainment purposes only.)
    for _ in 0..msg_bytes.len() {
        let random_byte = rng.gen();
        random_bytes.push(random_byte);
    }
    let encrypted_bytes = xor(msg_bytes, &random_bytes);
    share1.write(&random_bytes);
    share2.write(&encrypted_bytes);
}
