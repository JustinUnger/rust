#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
    }
}

type MD5Sum = [u32; 2];
type MD5Block = [u32; 16];

struct MD5State {
	a: u32,
	b: u32,
	c: u32,
	d: u32,
}

impl MD5State {
	fn new() -> Self {
		let (a,b,c,d) = (0,0,0,0);
		MD5State { a, b, c, d }
	}
}

fn md5(msg: &[u8]) -> MD5Sum {
	[0, 0]
}

// pad to 512bit/64byte/16words (32bit)

fn md5_block(msg: &[u8]) -> [u8; 64] {
	let mut v: Vec<u8> = Vec::with_capacity(64); 

	for &b in msg {
		v.push(b);		
	}
	[0; 64]
}
