fn lines(s: &str) -> Vec<&str> {
    let mut lines = vec![];
    let mut begin = 0;
    for (i,c) in s.chars().enumerate() {
        if c == '\n' {
            lines.push(&s[begin..i]);
            begin = i + 1;
        } 
    }   
    lines
}

struct Liner<'a> {
    s: &'a str,
}

impl<'a> Liner<'a> {
    fn new(is: &str) -> Liner {
        Liner { s: is }
    }
} 

impl<'a> Iterator for Liner<'a> {
	type Item = &'a str;
	
	fn next(&mut self) -> Option<Self::Item> {
		for (i,c) in self.s.chars().enumerate() {
			if c == '\n' {
				let p = &self.s[0..i];
				self.s = &self.s[i+1..];
				return Some(p);
			}
		}
		None
	}
}

fn main() {
    let s = "hello\nworld!\nfoo";
	let l = Liner::new(s);
	let vl: Vec<&str> = l.collect();

    println!("{:?}", lines(s));
	println!("{:?}", vl);

}
