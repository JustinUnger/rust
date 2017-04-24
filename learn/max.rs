fn max_copy<T: PartialOrd<T> + Copy>(xs: &[T]) -> T {
	let mut maximum = xs[0]; // maximum: T
    for &x in xs {           // x: T
    	if x > maximum {
        	maximum = x;
        }
    }
    maximum
}

fn max_clone<T: PartialOrd<T> + Clone>(xs: &[T]) -> T {
	let mut maximum = xs[0].clone(); // maximum: T
	for x in xs {                    // x: &T, xs: &[T]
		if *x > maximum {
			maximum = x.clone();
		}
	}
	maximum
}


/* better! no copy/clone required */

fn max_ref<T: PartialOrd<T>>(xs: &[T]) -> &T {
	let mut m = &xs[0]; // m: &T
	for x in xs {       // x: &T, xs: &[T]
		if *x > *m {
			m = x;
		}
	}
	m
}

/* use where clause for trait bounds */
fn max_ref1<T>(xs: &[T]) -> &T
where T: PartialOrd<T>
{
	let mut m = &xs[0];
	for x in xs {       // x: &T, xs: &[T]
		if *x > *m {
			m = x;
		}
	}
	m
}

fn main() {
	let a = vec![1,2,3];
	let b = [22,1,0,25,-1,0];
	let c = vec!['f','o','o','b','a','r'];
	let d = "hello world!";

	println!("max_copy({:?})={}",b,max_copy(&b));
	println!("max_clone({:?})={}",b,max_clone(&b));
	println!("max_ref({:?})={}",b,max_ref(&b));
}
