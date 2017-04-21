use std::env::args;

fn main() {
  let mut args = args();

  let steps = match args.nth(1) {
	Some(v) => v,
	None => panic!("more args plz")
  };

  let steps = steps.trim().parse().expect("please type a number");

  let tries = [collatz_steps(steps), collatz_steps2(steps), collatz_steps3(steps), collatz_steps4(steps)];
  println!("{:?}", tries);
}

fn collatz(n: isize) -> isize {
  if n == 1 { return 0; }
  match n % 2 {
    0 => { 1 + collatz(n/2) }
    _ => { 1 + collatz(n*3+1) }
  }
}

fn collatz_steps(steps: isize) -> isize {
  let mut i = 1;
  loop {
    if collatz(i) == steps {
      break;
    }
    i = i + 1;
  }
  i
}

fn collatz_steps2(steps: isize) -> isize {
  let mut i = 1;
  while collatz(i) != steps {
    i = i + 1;
  }
  i
}

fn collatz_steps3(steps: isize) -> isize {
  fn go(steps: isize, guess: isize) -> isize {
    let try = collatz(guess);
    if try == steps {
      return guess
    } else {
      go(steps, guess+1)
    }
  }
  go(steps,1)
}

fn collatz_steps4(steps: isize) -> isize {
	for guess in 1..isize::max_value() {
		let try = collatz(guess);
		if try == steps {
			return guess;
		}
	}
	0
}
