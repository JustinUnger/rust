fn main() {

    fn foo_mut(mut v: Vec<i32>) -> Vec<i32> {
        for x in &mut v {
            *x = *x + 1
        }
        v
    }

    fn sum(v: &[i32]) -> i32 {
        let mut tot: i32 = 0;
        for x in v {
            tot += *x
        }
        tot
    }

    fn sum_r(v: &[i32]) -> i32 {
        match v.split_first() {
            None => 0,
            Some((h, t)) => *h + sum_r(t),
        }
    }

    let xs = [1, 2, 3];
    let sum = sum(&xs);
    println!("sum {}", sum);
    let sumr = sum_r(&xs);
    println!("sumr {}", sumr);
    println!("{:?}", xs);

/*
    let xs = foo(&xs);
    println!("{:?}", xs);
*/

    let xs = vec![1, 2, 3];
    let ys = foo_mut(xs);
    //println!("{:?}", xs);
}

