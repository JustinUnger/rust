fn main() {
	let p = vec![1,2,3];
	let q = increment(p);
	println!("{:?}", q);

	let mut p = vec![1,2,3];
	incrementMut(&mut p);
	println!("{:?}", p);
}

fn increment(v: Vec<isize>) -> Vec<isize> {
	let mut nv = vec![];
	for x in v {
		nv.push(x+1);
	}
	nv
}

fn incrementMut(v: &mut Vec<isize>) {
	for x in v {
		*x = *x + 1;
	}
}	
