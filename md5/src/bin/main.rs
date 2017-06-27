extern crate md5;

fn main() {
	let prefix = "ojvtpuvg";
	
	for i in 0 .. u32::max_value() {
		let key = format!("{}{}", prefix, i);
		let digest = md5::md5_sum(key.as_bytes());
		
		if digest.starts_with("00000") {
			println!("key {} digest {}", key, digest);
		}
	}	

	println!("exhausted search");
}

