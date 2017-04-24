fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i,&c) in bytes.iter().enumerate() {
        if c == b' ' {   
            return &s[..i]
        }
    }
    &s
}   

fn main() {

    println!("{}", first_word(&String::from("Hello World!")));

}

fn max<T: PartialOrd<T> + Copy>(xs: &Vec<T>) -> T {
        let mut maximum = xs[0];
        for &x in xs {
            if x > maximum {
                maximum = x;
            }
        }
        maximum
    }
} 
