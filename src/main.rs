fn is_cons(c: char) -> bool {
    match c {
        'a' | 'e' | 'i' | 'o' | 'u' => false,
        'y' => false,
        _   => true,
    }
}

pub fn main() { 
    
    let test = "cbdfgacaca";
    let mut s = test.trim_left_matches(is_cons);
    let mut n = 0;

    loop {
        println!("{}", s);

        s = s.trim_left_matches(|c| !is_cons(c));

        if s.len() > 0 {
            n += 1;
        } else { 
            println!("{}", n);
            break;
        }
        println!("{}", s);

        s = s.trim_left_matches(|c| is_cons(c));
    }
}
