
use List::{Cons, Nil};

fn main() {

    let list = Cons(1,Box::new(Cons(2,Box::new(Nil))));

}

enum List<T> {
    Cons(T, Box<List<T>>),
    Nil
}


