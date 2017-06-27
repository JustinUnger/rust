#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn internal() {
		assert_eq!(u32_to_u8(0x1234_5678),[0x78,0x56,0x34,0x12]);
		assert_eq!(u8le_to_u32(&[0x78,0x56,0x34,0x12]), &0x1234_5678);
		let x: u32 = 0xffff_ffff;
		let res = Wrapping(x) + Wrapping(1);
		assert_eq!(res.0, 0);
	}

    #[test]
    fn it_works() {
	let null_md5 = md5_sum("".as_bytes());	
	assert_eq!(null_md5,"d41d8cd98f00b204e9800998ecf8427e");

	let a_md5 = md5_sum("a".as_bytes());
	assert_eq!(a_md5, "0cc175b9c0f1b6a831c399e269772661");

	let abc_md5 = md5_sum("abc".as_bytes());
	assert_eq!(abc_md5, "900150983cd24fb0d6963f7d28e17f72");
	
	let d1 = md5_sum("message digest".as_bytes());
	assert_eq!(d1, "f96b697d7cb7938d525a2f31aaf161d0");

	let d2 = md5_sum("abcdefghijklmnopqrstuvwxyz".as_bytes());
	assert_eq!(d2, "c3fcd3d76192e4007dfb496cca67e13b");

	let d3 = md5_sum("ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789".as_bytes());
	assert_eq!(d3,"d174ab98d277d9f5a5611c2c9f419d9f");
	
	let d4 = md5_sum("12345678901234567890123456789012345678901234567890123456789012345678901234567890".as_bytes());
	assert_eq!(d4,"57edf4a22be3c955ac49da2e2107b67a");
    }
}

use std::num::Wrapping;

#[derive(Debug,Clone,Copy)]
struct MD5State {
	a: Wrapping<u32>,
	b: Wrapping<u32>,
	c: Wrapping<u32>,
	d: Wrapping<u32>,
}

impl MD5State {
	fn new() -> Self {
		MD5State { a: INIT_A, b: INIT_B, c: INIT_C, d: INIT_D }
	}

	fn _show(&self) -> String {
		format!("MD5State ( a: {:08x} b: {:08x} c: {:08x}, d: {:08x} )",
			self.a, self.b, self.c, self.d)
	}

	fn digest(&self) -> String {
		let mut v: Vec<u8> = Vec::with_capacity(16);
		
		v.extend_from_slice(&u32_to_u8(self.a.0));
		v.extend_from_slice(&u32_to_u8(self.b.0));
		v.extend_from_slice(&u32_to_u8(self.c.0));
		v.extend_from_slice(&u32_to_u8(self.d.0));
	
		v.into_iter().map(|c| format!("{:02x}", c)).collect()	
	}

	fn f(&self, round: u32) -> (u32, u32) {
		( (self.b.0 & self.c.0) | ( (!self.b.0) & self.d.0), round)
	}

	fn g(&self, round: u32) -> (u32, u32) {
		( (self.d.0 & self.b.0) | ( (!self.d.0) & self.c.0), (5*round+1) % 16)
	}

	fn h(&self, round: u32) -> (u32, u32) {
		( self.b.0 ^ self.c.0 ^ self.d.0 , (3*round+5) % 16)
	}

	fn i(&self, round: u32) -> (u32,u32) {
		( self.c.0 ^ (self.b.0 | (! self.d.0)) , (7*round) % 16)
	}

	fn round(&mut self, block: Vec<&u32>) {
		assert!(block.len() == 16);

		let mut round_state = *self;

		for i in 0..64 {
			let (f, g) = match i {
				00 ... 15 => round_state.f(i), 
				16 ... 31 => round_state.g(i),
				32 ... 47 => round_state.h(i),
				_         => round_state.i(i),
			};

			assert!(g < 16);

			let d_temp = round_state.d;
			round_state.d = round_state.c;
			round_state.c = round_state.b;
			round_state.b = round_state.b + foo(round_state.a, Wrapping(f), i as usize, Wrapping(*block[g as usize]));
			round_state.a = d_temp;	
		}

		self.a += round_state.a;
		self.b += round_state.b;
		self.c += round_state.c;
		self.d += round_state.d;
	}
}

const INIT_A: Wrapping<u32> = Wrapping(0x6745_2301);
const INIT_B: Wrapping<u32> = Wrapping(0xefcd_ab89);
const INIT_C: Wrapping<u32> = Wrapping(0x98ba_dcfe);
const INIT_D: Wrapping<u32> = Wrapping(0x1032_5476);

const PER_ROUND_SHIFT: [u8; 64] = [
	7, 12, 17, 22,  7, 12, 17, 22,  7, 12, 17, 22,  7, 12, 17, 22,
	5,  9, 14, 20,  5,  9, 14, 20,  5,  9, 14, 20,  5,  9, 14, 20,
	4, 11, 16, 23,  4, 11, 16, 23,  4, 11, 16, 23,  4, 11, 16, 23,
	6, 10, 15, 21,  6, 10, 15, 21,  6, 10, 15, 21,  6, 10, 15, 21,
]; 

const K: [u32; 64] = [
	0xd76aa478, 0xe8c7b756, 0x242070db, 0xc1bdceee,
	0xf57c0faf, 0x4787c62a, 0xa8304613, 0xfd469501,
	0x698098d8, 0x8b44f7af, 0xffff5bb1, 0x895cd7be,
	0x6b901122, 0xfd987193, 0xa679438e, 0x49b40821,
	0xf61e2562, 0xc040b340, 0x265e5a51, 0xe9b6c7aa,
	0xd62f105d, 0x02441453, 0xd8a1e681, 0xe7d3fbc8,
	0x21e1cde6, 0xc33707d6, 0xf4d50d87, 0x455a14ed,
	0xa9e3e905, 0xfcefa3f8, 0x676f02d9, 0x8d2a4c8a,
	0xfffa3942, 0x8771f681, 0x6d9d6122, 0xfde5380c,
	0xa4beea44, 0x4bdecfa9, 0xf6bb4b60, 0xbebfbc70,
	0x289b7ec6, 0xeaa127fa, 0xd4ef3085, 0x04881d05,
	0xd9d4d039, 0xe6db99e5, 0x1fa27cf8, 0xc4ac5665,
	0xf4292244, 0x432aff97, 0xab9423a7, 0xfc93a039,
	0x655b59c3, 0x8f0ccc92, 0xffeff47d, 0x85845dd1,
	0x6fa87e4f, 0xfe2ce6e0, 0xa3014314, 0x4e0811a1,
	0xf7537e82, 0xbd3af235, 0x2ad7d2bb, 0xeb86d391,
];

fn u32_to_u8(x: u32) -> [u8;4] {
	let a = ((x >> 24) & 0xff) as u8;
	let b = ((x >> 16) & 0xff) as u8;
	let c = ((x >> 8)  & 0xff) as u8;
	let d = (x & 0xff) as u8;

	[d, c, b, a]	
}

use std::io::Read;

pub fn md5_sum<T: Read>(mut reader: T) -> String {
	let mut md5_state = MD5State::new();
	let mut buf = [0 as u8; 64];
	
	let mut msg_bytes: usize = 0;
	loop {
		let bytes_read = reader.read(&mut buf).unwrap();
		msg_bytes += bytes_read;
	
		if bytes_read < 64 {
			break;
		}
		md5_state = md5_op(md5_state, &buf);
	}

	let msg_bits: u64 = (msg_bytes * 8) as u64;
	let mut msg_bits_hi: u32 = (msg_bits >> 32) as u32;
	let mut msg_bits_lo: u32 = msg_bits as u32; 
	
	let rem = msg_bytes % 64;
	let pad_bytes = 64-(msg_bytes % 64);	

	if pad_bytes < 9 {
		buf[rem] = 0x80;
		for i in rem+1 .. 64 {
			buf[i] = 0;
		}
		md5_state = md5_op(md5_state, &buf);
		for i in 0 .. 56 {
			buf[i] = 0;
		}
	} else {
		buf[rem] = 0x80;
		for i in rem+1 .. 56 {
			buf[i] = 0;
		}
	}

	for i in 56 .. 60 {
		buf[i] = msg_bits_lo as u8;
		msg_bits_lo >>= 8;
	}
	for i in 60 .. 64 {
		buf[i] = msg_bits_hi as u8;
		msg_bits_hi >>= 8;
	}

	md5_state = md5_op(md5_state, &buf);
	
	md5_state.digest()
}

fn md5_op(mut md5_state: MD5State, chunk: &[u8]) -> MD5State {
	for block in MD5Blocker(chunk) {
		md5_state.round(block);
	}
	
	md5_state
}

fn foo(a: Wrapping<u32>, f: Wrapping<u32>, i: usize, m: Wrapping<u32>) -> Wrapping<u32> {
	assert!(i < 64);

	let k = Wrapping(K[i]);
	let shift = PER_ROUND_SHIFT[i] as u32;
	let temp = a + f + k + m;

	Wrapping(temp.0.rotate_left(shift))
}

type MD5Block<'a> = Vec<&'a u32>;

struct MD5Blocker<'a>(pub &'a [u8]);

impl<'a> Iterator for MD5Blocker<'a> {
	type Item = Vec<&'a u32>;

	fn next(&mut self) -> Option<MD5Block<'a>> {
		let msg = self.0;
		if msg.is_empty() {
			return None;
		}
	
		let (next_block, new_msg) = block(msg);
		assert!(next_block.len() == 16);
		self.0 = new_msg;
		Some(next_block)
	}
}

fn u8le_to_u32(bytes: &[u8]) -> &u32 {
	assert!(bytes.len() >= 4);
	
	let ptr_u8 = bytes.as_ptr();
	let ptr_u32 = ptr_u8 as *const u32;
	
	unsafe { ptr_u32.as_ref().unwrap() }
}

// byte vector to blocks of sixteen 32 bit words (512bits)
fn block(mut msg: &[u8]) -> (MD5Block,&[u8]) {
	assert!(msg.len() == 64);

	let mut v: MD5Block = Vec::with_capacity(16);

	for _ in 0..16 {
		let x = u8le_to_u32(&msg[0..4]);
		v.push(x);
		msg = &msg[4..];
	}

	(v, msg)
}

